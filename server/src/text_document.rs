use crate::from_lsp;
use async_lsp::lsp_types as lsp;
use bord_sqlite3_parser::{parse, SqliteUntypedCst};
use line_index::LineIndex;

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
}

impl TextDocument {
    pub fn new(doc_version: i32, contents: String) -> Self {
        let cst = parse(&contents);

        TextDocument {
            line_index: LineIndex::new(&contents),
            doc_version,
            contents,
            cst,
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

        Ok(())
    }
}
