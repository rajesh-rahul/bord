use crate::{flycheck::check_statement, from_lsp};
use async_lsp::lsp_types as lsp;
use bord_sqlite3_parser::{
    incremental_parse, parse, ModifiedBranchesInfo, NodeId, SqliteTreeKind, SqliteUntypedCst,
    TextPatch, TextPatchKind,
};
use line_index::{LineIndex, TextRange};
use rusqlite::Connection;

#[derive(Debug)]
pub struct TextDocument {
    /// When we receive 'location' in text document via LSP, it is in
    /// (line, column) format but we want it as an offset from the start of the file
    /// `line_index` takes care of this
    pub(crate) line_index: LineIndex,
    /// Document version is provided by the LSP client. Protocol guarantees the number will
    /// increase for any change to the contents.
    pub(crate) doc_version: i32,
    /// Textual data of the text document
    pub(crate) contents: String,
    // TODO: Add lsp's language id info
    pub(crate) cst: SqliteUntypedCst,

    pub(crate) errors: Vec<Vec<lsp::Diagnostic>>,
    // pub(crate) flycheck_errors: Vec<Option<FlycheckError>>,
}

impl TextDocument {
    pub fn new(doc_version: i32, contents: String) -> Self {
        let cst = parse(&contents);

        TextDocument {
            line_index: LineIndex::new(&contents),
            doc_version,
            contents,
            errors: vec![vec![]; cst.num_branches() + 1],
            cst,
        }
    }

    pub fn apply_changes(
        &mut self,
        doc_version: i32,
        changes: Vec<lsp::TextDocumentContentChangeEvent>,
    ) -> anyhow::Result<ModifiedBranchesInfo> {
        // The given version by LSP represents the document AFTER all changes in the vec are
        // applied
        anyhow::ensure!(self.doc_version <= doc_version, "Unexpected doc version");
        self.doc_version = doc_version;

        // Seems impossible, but we add a check just in case
        if changes.is_empty() {
            return Ok(ModifiedBranchesInfo {
                splice_range: 0..0,
                num_new_branches: 0,
            });
        }

        let more_than_one_change = changes.len() > 1;
        let mut text_patch = None;

        for change in changes {
            if let Some(lsp_range) = change.range {
                let range = from_lsp::text_range(&self.line_index, lsp_range)?;

                self.contents
                    .replace_range(std::ops::Range::<usize>::from(range), &change.text);

                if range.start() == range.end() && text_patch.is_none() {
                    text_patch = Some(TextPatch {
                        relex_start: (),
                        affected_node_byte_len: (),
                        start: range.start().into(),
                        size: change.text.len(),
                        kind: TextPatchKind::Insert,
                    });
                } else if text_patch.is_none() {
                    text_patch = Some(TextPatch {
                        relex_start: (),
                        affected_node_byte_len: (),
                        start: range.start().into(),
                        size: change.text.len(),
                        kind: TextPatchKind::Replace {
                            end: range.end().into(),
                        },
                    });
                }
            } else {
                // No range indicates the given text represents the entire document
                self.contents = change.text;
            };
            // Rebuild index because a change may span multiple lines
            self.line_index = LineIndex::new(&self.contents);
        }

        // Our incremental parser only supports applying each change one by one. In most cases,
        // it will be faster to just to apply all changes to text in one go and running the parser
        // on the entire text. NOTE: Text patch is none if the entire document was changed
        if text_patch.is_none() || more_than_one_change {
            self.cst = parse(&self.contents);

            return Ok(ModifiedBranchesInfo {
                splice_range: 0..self.cst.num_branches() + 1,
                num_new_branches: self.cst.num_branches(),
            });
        }

        let text_patch = text_patch.expect("We already returned if text_patch was None");

        let start_time = std::time::Instant::now();
        let text_patch = self.cst.updated_text_patch(text_patch);

        let relex_input = &self.contents[text_patch.relex_start..];
        let incr_cst = incremental_parse(relex_input, text_patch);
        let elapsed = start_time.elapsed();
        let mod_info = self.cst.merge_cst(incr_cst, text_patch);

        eprintln!(
            "Incremental: {}, mod_info: {mod_info:?}",
            elapsed.as_micros()
        );

        #[cfg(test)]
        {
            let start = std::time::Instant::now();
            let new_cst_string = self.cst.root().to_text_with_capacity(self.contents.len());
            assert_eq!(new_cst_string, self.contents);
            let elapsed = start.elapsed();
            eprintln!("String compare: {}", elapsed.as_micros());

            let start = std::time::Instant::now();
            let old_cst = std::mem::replace(&mut self.cst, parse(&self.contents));
            let elapsed = start.elapsed();
            eprintln!("Regular: {}", elapsed.as_micros());
            assert_eq!(old_cst, self.cst);
        }

        Ok(mod_info)
    }

    pub fn update_errors(
        &mut self,
        mod_info: ModifiedBranchesInfo,
        conn: &Connection,
    ) -> anyhow::Result<()> {
        let start = mod_info.splice_range.start;
        let end = start + mod_info.num_new_branches;

        let updated_errors = (start..end).map(|branch_id| {
            let root_node = self.cst.node(NodeId::new(branch_id, 0));

            let mut errors = Vec::new();

            let parse_errors = root_node
                .me_and_descendants()
                .filter(|it| it.error().is_some())
                .map(|it| {
                    let range = from_lsp::node_lsp_range(&self.line_index, &it).unwrap();

                    lsp::Diagnostic {
                        range,
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: format!("{:?}", it.error().unwrap()),
                        source: Some("bordsql".into()),
                        ..Default::default()
                    }
                });

            errors.extend(parse_errors);

            // Only do fly check if we do not have any parse errors and if the current root node is a statement
            if errors.is_empty() && root_node.tree() == Some(SqliteTreeKind::Statement) {
                let text_start = root_node.start_pos_skip_trivia();
                let text_end = root_node.end_pos_skip_trivia();

                let range = from_lsp::lsp_range(
                    &self.line_index,
                    TextRange::new(text_start.try_into().unwrap(), text_end.try_into().unwrap()),
                )
                .unwrap();

                if let Err(err) = check_statement(conn, &self.contents[text_start..text_end]) {
                    errors.push(lsp::Diagnostic {
                        range,
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: err,
                        source: Some("bordsql".into()),
                        ..Default::default()
                    });
                }
            }

            errors
        });

        // The extend is required when multi-cursor edits happen
        self.errors.extend(
            std::iter::repeat(Vec::new())
                .take(mod_info.splice_range.end.saturating_sub(self.errors.len())),
        );
        self.errors.splice(mod_info.splice_range, updated_errors);

        Ok(())
    }
}
