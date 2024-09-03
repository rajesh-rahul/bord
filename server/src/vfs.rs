use crate::from_lsp;
use hashbrown::HashMap;
use line_index::LineIndex;
use lsp_types as lsp;
use yukon_sqlite3_parser::{parse, SqliteParseError, SqliteUntypedAst};
use std::ops::Range as StdRange;

#[derive(Debug)]
pub struct TextDocument {
    /// When we receive 'location' in text document via LSP, it is in
    /// (line, column) format but we want it as an offset from the start of the file
    /// `line_index` takes care of this
    line_index: LineIndex,
    /// Document version is provided by the LSP client. Protocol guarantees the number will
    /// increase for any change to the contents.
    doc_version: i32,
    /// Textual data of the text document
    contents: String,
    // TODO: Add lsp's language id info
    ast: SqliteUntypedAst,
}

impl TextDocument {
    pub fn new(doc_version: i32, contents: String) -> (Self, Vec<SqliteParseError>) {
        let (ast, errors) = parse(&contents);

        let new_doc = TextDocument {
            line_index: LineIndex::new(&contents),
            doc_version,
            contents,
            ast,
        };

        (new_doc, errors)
    }

    pub fn apply_changes(
        &mut self,
        doc_version: i32,
        changes: Vec<lsp::TextDocumentContentChangeEvent>,
    ) -> anyhow::Result<Vec<SqliteParseError>> {
        for change in changes {
            if let Some(lsp_range) = change.range {
                let range = from_lsp::text_range(&self.line_index, lsp_range)?;

                self.contents
                    .replace_range(StdRange::<usize>::from(range), &change.text);
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

        let (ast, errors) = parse(&self.contents);
        self.ast = ast;

        Ok(errors)
    }
}

#[derive(Default, Debug)]
pub struct Vfs {
    // TODO: Use SQLite for this if using too much memory?
    files: HashMap<lsp::Url, TextDocument>,
}

impl Vfs {
    pub fn add_new_text_document(
        &mut self,
        data: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<Vec<SqliteParseError>> {
        let doc_url = data.text_document.uri;
        let doc_version = data.text_document.version;

        if let Some(_) = self.files.get(&doc_url) {
            anyhow::bail!("Document was already opened by editor");
        }

        let (new_doc, errors) = TextDocument::new(doc_version, data.text_document.text);
        self.files.insert(doc_url, new_doc);

        Ok(errors)
    }

    pub fn close_text_document(
        &mut self,
        data: lsp::DidCloseTextDocumentParams,
    ) -> anyhow::Result<()> {
        let doc_url = data.text_document.uri;

        if self.files.remove(&doc_url).is_none() {
            anyhow::bail!("Document was never opened by editor");
        }

        Ok(())
    }

    pub fn get_text_document_contents(&self, doc_url: &lsp::Url) -> Option<&str> {
        self.files.get(doc_url).map(|doc| doc.contents.as_str())
    }

    pub fn get_line_index(&self, doc_url: &lsp::Url) -> Option<&LineIndex> {
        self.files.get(doc_url).map(|doc| &doc.line_index)
    }

    pub fn get_mut_text_document(&mut self, doc_url: &lsp::Url) -> Option<&mut TextDocument> {
        self.files.get_mut(doc_url)
    }

    pub fn get_text_document(&self, doc_url: &lsp::Url) -> Option<&TextDocument> {
        self.files.get(doc_url)
    }
}
