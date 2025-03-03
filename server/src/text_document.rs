use crate::{from_lsp, BordLangServer};
use async_lsp::lsp_types::{self as lsp};
use bord_sqlite3_parser::{
    batch, incr, incremental_parse2, parse, parse_with_abs_pos, CstNodeTrait, CstTrait, TextPatch,
    TextPatchKind,
};
use line_index::LineIndex;
use rusqlite::Connection;
use text_size::{TextRange, TextSize};

#[derive(Debug)]
pub enum TextDocumentCstKind {
    FullSqlFile(incr::IncrSqlCst),
    NonSqlFile {
        csts: Vec<batch::SqlCst>,
        lang_id: String,
    },
}

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
    pub(crate) cst: TextDocumentCstKind,

    pub(crate) errors: Vec<lsp::Diagnostic>,
    // pub(crate) flycheck_errors: Vec<Option<FlycheckError>>,
}

impl Default for TextDocumentCstKind {
    fn default() -> Self {
        TextDocumentCstKind::NonSqlFile {
            csts: Vec::new(),
            lang_id: "".into(),
        }
    }
}

impl TextDocumentCstKind {
    pub fn new(server: &BordLangServer, lang_id: String, contents: &str) -> Self {
        if lang_id == "sql" {
            let cst = parse(&contents);
            TextDocumentCstKind::FullSqlFile(cst)
        } else if let Some(match_pattern) = server.config.match_patterns.get(&lang_id) {
            let csts = match_pattern
                .match_on_haystack(&contents)
                .map(|it| {
                    parse_with_abs_pos::<batch::SqlCst>(
                        TextSize::from(it.start() as u32),
                        it.as_str(),
                    )
                })
                .collect();

            TextDocumentCstKind::NonSqlFile { csts, lang_id }
        } else {
            TextDocumentCstKind::NonSqlFile {
                csts: Vec::new(),
                lang_id,
            }
        }
    }
}

impl TextDocument {
    pub fn new(server: &BordLangServer, params: lsp::DidOpenTextDocumentParams) -> Self {
        let lang_id = params.text_document.language_id;
        let contents = params.text_document.text;

        let cst = TextDocumentCstKind::new(server, lang_id, &contents);

        TextDocument {
            line_index: LineIndex::new(&contents),
            doc_version: params.text_document.version,
            contents,
            errors: vec![],
            cst,
        }
    }

    pub fn apply_changes(
        &mut self,
        server: &BordLangServer,
        doc_version: i32,
        params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        // The given version by LSP represents the document AFTER all changes in the vec are
        // applied
        anyhow::ensure!(self.doc_version <= doc_version, "Unexpected doc version");
        self.doc_version = doc_version;

        match &mut self.cst {
            TextDocumentCstKind::FullSqlFile(cst) => {
                for change in params.content_changes {
                    if let Some(lsp_range) = change.range {
                        let range = from_lsp::text_range(&self.line_index, lsp_range)?;

                        self.contents
                            .replace_range(std::ops::Range::<usize>::from(range), &change.text);

                        let text_patch = TextPatch {
                            relex_start: (),
                            affected_node_byte_len: (),
                            start: range.start(),
                            size: TextSize::from(change.text.len() as u32),
                            kind: if range.start() == range.end() {
                                TextPatchKind::Insert
                            } else {
                                TextPatchKind::Replace {
                                    end: range.end().into(),
                                }
                            },
                        };

                        let start = std::time::Instant::now();
                        let text_patch = cst.updated_text_patch(text_patch);
                        let relex_input = &self.contents[text_patch.relex_start.into()..];
                        let new_cst = incremental_parse2(relex_input, text_patch);
                        cst.merge_cst(new_cst, text_patch);
                        tracing::info!("Incremental Parse Time: {}", start.elapsed().as_micros());
                        // eprintln!("{cst}");
                        let start = std::time::Instant::now();
                        let batch_cst: batch::SqlCst = parse(&self.contents);
                        eprintln!("Normal Parse Time: {}", start.elapsed().as_micros());
                        // eprintln!("{batch_cst}");
                        assert_eq!(cst.root().comparable(), batch_cst.root().comparable());
                    }
                    // No range indicates the given text represents the entire document
                    else {
                        self.contents = change.text;

                        *cst = parse(&self.contents);
                    }

                    // Rebuild index because a change may span multiple lines
                    self.line_index = LineIndex::new(&self.contents);
                }
            }
            TextDocumentCstKind::NonSqlFile { lang_id, .. } => {
                for change in params.content_changes {
                    if let Some(lsp_range) = change.range {
                        let range = from_lsp::text_range(&self.line_index, lsp_range)?;

                        self.contents
                            .replace_range(std::ops::Range::<usize>::from(range), &change.text);
                    }
                    // No range indicates the given text represents the entire document
                    else {
                        self.contents = change.text;
                    };

                    // Rebuild index because a change may span multiple lines
                    self.line_index = LineIndex::new(&self.contents);
                }

                self.cst = TextDocumentCstKind::new(server, lang_id.clone(), &self.contents)
            }
        }

        Ok(())
    }

    pub fn update_errors(&mut self, conn: &Connection) -> anyhow::Result<()> {
        let errors = match &self.cst {
            TextDocumentCstKind::FullSqlFile(incr_sql_cst) => incr_sql_cst
                .root()
                .me_and_descendants()
                .filter(|it| it.error().is_some())
                .map(|it| {
                    let mut range = from_lsp::node_lsp_range(&self.line_index, &it).unwrap();
                    range.end = range.start;
                    lsp::Diagnostic {
                        range,
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        message: format!("{:?}", it.error().unwrap()),
                        source: Some("bordsql".into()),
                        ..Default::default()
                    }
                })
                .collect(),
            TextDocumentCstKind::NonSqlFile { csts, lang_id } => csts
                .iter()
                .flat_map(|it| {
                    it.root()
                        .me_and_descendants()
                        .filter(|it| it.error().is_some())
                        .map(|it| {
                            let range = from_lsp::lsp_range(
                                &self.line_index,
                                TextRange::new(
                                    it.start_pos_skip_trivia(),
                                    it.end_pos_skip_trivia(),
                                )
                                .into(),
                            )
                            .unwrap();

                            lsp::Diagnostic {
                                range,
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                message: format!("{:?}", it.error().unwrap()),
                                source: Some("bordsql".into()),
                                ..Default::default()
                            }
                        })
                })
                .collect(),
        };
        // The extend is required when multi-cursor edits happen
        self.errors = errors;

        Ok(())
    }
}
