use capabilities::server_capabilities;
mod capabilities;
mod features;
mod from_lsp;
mod text_document;
mod vfs;

// pub type Result<T> = anyhow::Result<T>;

#[derive(Debug)]
struct BordLangServer {
    client: Client,
    vfs: vfs::Vfs,
}

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types as lsp;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[tower_lsp::async_trait]
impl LanguageServer for BordLangServer {
    async fn initialize(&self, _: lsp::InitializeParams) -> Result<lsp::InitializeResult> {
        Ok(lsp::InitializeResult {
            server_info: None,
            capabilities: server_capabilities(),
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        self.vfs.add_new_text_document(params);
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let doc_url = params.text_document.uri.clone();
        let version = params.text_document.version;

        let Some(mut doc) = self.vfs.files.get_mut(&doc_url) else {
            return;
        };

        if let Err(err) = doc.apply_changes(version, params.content_changes) {
            tracing::warn!("{}", err);
            return;
        }

        let diagnostics = doc.diagnostics.clone();
        self.client
            .publish_diagnostics(doc_url, diagnostics, None)
            .await;
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        self.vfs.close_text_document(params);
    }

    async fn completion(
        &self,
        params: lsp::CompletionParams,
    ) -> Result<Option<lsp::CompletionResponse>> {
        let doc_pos = params.text_document_position;
        let Some(document) = self.vfs.files.get(&doc_pos.text_document.uri) else {
            tracing::warn!(
                "Recieved completion request for non-existent document: {}",
                doc_pos.text_document.uri
            );
            return Ok(None);
        };

        // TODO: DO this properly
        let Ok(cursor) = from_lsp::offset(
            &document.line_index,
            doc_pos.position.line,
            doc_pos.position.character,
        ) else {
            tracing::error!("Unable to convert lsp text position");
            return Ok(None);
        };

        let ast = &document.cst;

        let completions = features::create_completion_context(ast, cursor)
            .into_iter()
            .map(|it| lsp::CompletionItem {
                label: it,
                kind: Some(lsp::CompletionItemKind::KEYWORD),
                ..Default::default()
            })
            .collect();

        Ok(Some(lsp::CompletionResponse::Array(completions)))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .finish();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| BordLangServer {
        client,
        vfs: Default::default(),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
