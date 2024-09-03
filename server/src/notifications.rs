use crate::features;
use crate::utils::ConnectionExt;
use crate::YkConnection;
use crate::YkLangServer;
use crate::Result;
use anyhow::bail;
use lsp_server::ExtractError;
use lsp_server::Notification;

use lsp_types::notification as lsp;
use lsp_types::notification::Notification as _;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::Diagnostic;
use lsp_types::DiagnosticSeverity;
use lsp_types::PublishDiagnosticsParams;

pub fn handle(conn: &YkConnection, not: Notification, server: &mut YkLangServer) -> Result<()> {
    use lsp::Notification;
    conn.log_info(format!("Received {}", not.method));

    match not.method.as_str() {
        lsp::DidOpenTextDocument::METHOD => did_open_text_document(conn, not, server),
        lsp::DidChangeTextDocument::METHOD => did_change_text_document(conn, not, server),
        lsp::DidCloseTextDocument::METHOD => did_close_text_document(conn, not, server),
        _ => Ok(()),
    }
}

fn cast<N>(not: Notification) -> std::result::Result<N::Params, ExtractError<Notification>>
where
    N: lsp::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    not.extract(N::METHOD)
}

pub fn did_open_text_document(
    conn: &YkConnection,
    not: Notification,
    server: &mut YkLangServer,
) -> Result<()> {
    let params = cast::<lsp::DidOpenTextDocument>(not)?;

    let document_uri = params.text_document.uri.clone();
    server.vfs.add_new_text_document(params)?;
    eprint!("Document URI: {:?}", document_uri);
    features::publish_diagnosics(
        server
            .vfs
            .get_text_document_contents(&document_uri)
            .unwrap(),
    );

    Ok(())
}

pub fn did_change_text_document(
    conn: &YkConnection,
    not: Notification,
    server: &mut YkLangServer,
) -> Result<()> {
    let params = cast::<lsp::DidChangeTextDocument>(not)?;

    let document_uri = params.text_document.uri.clone();
    let version = params.text_document.version;

    let Some(doc) = server.vfs.get_mut_text_document(&document_uri) else {
        bail!("Unexpected documented")
    };

    let errors = doc.apply_changes(version, params.content_changes)?;

    let line_index = server.vfs.get_line_index(&document_uri).unwrap();

    let params = PublishDiagnosticsParams {
        uri: document_uri.clone(),
        diagnostics: errors
            .iter()
            .map(|err| {
                let start_pos = line_index.try_line_col(err.range.0.into()).unwrap();
                let end_pos = line_index.try_line_col((err.range.1).into()).unwrap();

                let start = lsp_types::Position {
                    line: start_pos.line,
                    character: start_pos.col,
                };
                let end = lsp_types::Position {
                    line: end_pos.line,
                    character: end_pos.col + 1,
                };
                Diagnostic {
                    range: lsp_types::Range { start, end },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.message.to_string(),
                    source: Some("yukonsql".into()),
                    ..Default::default()
                }
            })
            .collect(),
        version: Some(version),
    };

    conn.lsp_client
        .sender
        .send(lsp_server::Message::Notification(
            lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), params),
        ));

    Ok(())
}

pub fn did_close_text_document(
    conn: &YkConnection,
    not: Notification,
    server: &mut YkLangServer,
) -> Result<()> {
    let params = cast::<lsp::DidCloseTextDocument>(not)?;

    server.vfs.close_text_document(params)?;

    Ok(())
}
