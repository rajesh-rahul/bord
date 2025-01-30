use std::ops::ControlFlow;
use std::sync::Mutex;

use capabilities::server_capabilities;
mod capabilities;
mod features;
mod flycheck;
mod from_lsp;
mod text_document;
mod vfs;

use async_lsp::router::Router;
use async_lsp::lsp_types as lsp;
use async_lsp::lsp_types::notification as not;
use async_lsp::lsp_types::request as req;

#[derive(Debug)]
pub struct BordLangServer {
    client: async_lsp::ClientSocket,
    vfs: vfs::Vfs,
    flycheck_db: Mutex<rusqlite::Connection>,
}

impl BordLangServer {
    pub fn new(client: async_lsp::ClientSocket) -> Self {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../schema.sql"))
            .unwrap();

        BordLangServer {
            client,
            vfs: Default::default(),
            flycheck_db: Mutex::new(conn),
        }
    }
}

fn did_open_text_document(
    server: &mut BordLangServer,
    params: lsp::DidOpenTextDocumentParams,
) -> ControlFlow<Result<(), async_lsp::Error>> {
    server.vfs.add_new_text_document(params);

    ControlFlow::Continue(())
}

fn did_close_text_document(
    server: &mut BordLangServer,
    params: lsp::DidCloseTextDocumentParams,
) -> ControlFlow<Result<(), async_lsp::Error>> {
    server.vfs.close_text_document(params);

    ControlFlow::Continue(())
}

fn did_change_text_document(
    server: &mut BordLangServer,
    params: lsp::DidChangeTextDocumentParams,
) -> ControlFlow<Result<(), async_lsp::Error>> {
    let doc_url = params.text_document.uri.clone();
    let version = params.text_document.version;

    let Some(mut doc) = server.vfs.files.get_mut(&doc_url) else {
        tracing::warn!("{doc_url} not found in server");
        return ControlFlow::Continue(());
    };

    if let Err(err) = doc.apply_changes(version, params.content_changes) {
        tracing::warn!("{}", err);
        return ControlFlow::Continue(());
    }

    // TODO: TERRIBLE! connection and flycheck need more work
    let diagnostics = features::perform_diagnostics(&server.flycheck_db.lock().unwrap(), &doc);

    if let Err(err) =
        server
            .client
            .notify::<not::PublishDiagnostics>(lsp::PublishDiagnosticsParams {
                uri: doc_url,
                diagnostics,
                version: Some(doc.doc_version),
            })
    {
        return ControlFlow::Break(Err(err));
    }

    ControlFlow::Continue(())
}

fn completion(
    server: &mut BordLangServer,
    params: lsp::CompletionParams,
) -> Option<lsp::CompletionResponse> {
    let doc_pos = params.text_document_position;
    let Some(document) = server.vfs.files.get(&doc_pos.text_document.uri) else {
        tracing::warn!(
            "Recieved completion request for non-existent document: {}",
            doc_pos.text_document.uri
        );
        return None;
    };

    // TODO: DO this properly
    let Ok(cursor) = from_lsp::offset(
        &document.line_index,
        doc_pos.position.line,
        doc_pos.position.character,
    ) else {
        tracing::error!("Unable to convert lsp text position");
        return None;
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

    Some(lsp::CompletionResponse::Array(completions))
}

pub struct TickEvent;

pub fn router(client: async_lsp::ClientSocket) -> Router<BordLangServer> {
    let mut router = async_lsp::router::Router::new(BordLangServer::new(client));

    router
        .request::<req::Initialize, _>(|_, _| async move {
            Ok(lsp::InitializeResult {
                server_info: None,
                capabilities: server_capabilities(),
            })
        })
        .notification::<not::Initialized>(|_, _| ControlFlow::Continue(()))
        .notification::<not::DidChangeConfiguration>(|_, _| ControlFlow::Continue(()))
        .notification::<not::DidOpenTextDocument>(did_open_text_document)
        .notification::<not::DidChangeTextDocument>(did_change_text_document)
        .request::<req::Completion, _>(|s, p| {
            let completions = completion(s, p);

            async move { Ok(completions) }
        })
        .notification::<not::DidCloseTextDocument>(did_close_text_document)
        .unhandled_notification(|_, _| ControlFlow::Continue(()));

    router
}
