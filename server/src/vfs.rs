use crate::{text_document::TextDocument, BordLangServer};
use async_lsp::lsp_types as lsp;
use dashmap::DashMap;

#[derive(Default, Debug)]
pub struct Vfs {
    // TODO: Use SQLite for this if using too much memory?
    pub(crate) files: DashMap<lsp::Url, TextDocument>,
}

impl Vfs {
    pub fn add_new_text_document(
        &self,
        server: &BordLangServer,
        data: lsp::DidOpenTextDocumentParams,
    ) {
        let doc_url = data.text_document.uri.clone();
        let new_doc = TextDocument::new(server, data);

        if let Some(_) = self.files.insert(doc_url.clone(), new_doc) {
            tracing::warn!("{doc_url} already existed")
        }
    }

    pub fn close_text_document(&self, data: lsp::DidCloseTextDocumentParams) {
        let doc_url = data.text_document.uri;

        if self.files.remove(&doc_url).is_some() {
            tracing::warn!("{doc_url} not found")
        }
    }
}
