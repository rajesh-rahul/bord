use crate::from_lsp;
use bord_sqlite3_parser::{parse, SqliteParseError, SqliteUntypedCst};
use line_index::LineIndex;
use tower_lsp::lsp_types as lsp;

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

    pub(crate) diagnostics: Vec<lsp::Diagnostic>,
}

impl TextDocument {
    pub fn new(doc_version: i32, contents: String) -> Self {
        let cst = parse(&contents);

        TextDocument {
            line_index: LineIndex::new(&contents),
            doc_version,
            contents,
            cst,
            diagnostics: Vec::new(),
        }
    }

    pub fn apply_changes(
        &mut self,
        doc_version: i32,
        changes: Vec<lsp::TextDocumentContentChangeEvent>,
    ) -> anyhow::Result<()> {
        for change in changes {
            if let Some(lsp_range) = change.range {
                let range = from_lsp::text_range(&self.line_index, lsp_range)?;

                self.contents
                    .replace_range(std::ops::Range::<usize>::from(range), &change.text);
            } else {
                // No range indicates the given text represents the entire document
                self.contents = change.text;
            }

            // Rebuild index because a change may span multiple lines
            self.line_index = LineIndex::new(&self.contents);
        }

        // The given version by LSP represents the document AFTER all changes in the vec are
        // applied
        anyhow::ensure!(self.doc_version < doc_version, "Unexpected doc version");

        self.doc_version = doc_version;
        self.cst = parse(&self.contents);
        self.update_diagnostics();

        Ok(())
    }

    pub fn errors(&self) -> &[SqliteParseError] {
        self.cst.errors()
    }

    pub fn line_index(&self) -> &LineIndex {
        &self.line_index
    }

    fn update_diagnostics(&mut self) {
        self.diagnostics = self
            .errors()
            .iter()
            .map(|err| {
                let start_pos = self.line_index.try_line_col(err.range.0.into()).unwrap();
                let end_pos = self.line_index.try_line_col((err.range.1).into()).unwrap();

                let start = lsp::Position {
                    line: start_pos.line,
                    character: start_pos.col,
                };
                let end = lsp::Position {
                    line: end_pos.line,
                    character: end_pos.col + 1,
                };
                lsp::Diagnostic {
                    range: lsp::Range { start, end },
                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                    message: err.to_string(),
                    source: Some("bordsql".into()),
                    ..Default::default()
                }
            })
            .collect();
    }

    pub(crate) fn diagnostics(&self) -> &[lsp::Diagnostic] {
        &self.diagnostics
    }
}
