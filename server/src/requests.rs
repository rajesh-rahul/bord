use crate::from_lsp;
use crate::utils::ConnectionExt;
use crate::YkConnection;
use crate::YkLangServer;
use crate::Result;
use anyhow::bail;
use lsp_server::ExtractError;
use lsp_server::RequestId;
use lsp_server::Response;
use lsp_types::request as lsp;
use lsp_types::request::Completion;
use lsp_types::request::Request;
use lsp_types::CompletionItem;
use lsp_types::CompletionResponse;
pub fn handle(
    conn: &YkConnection,
    req: lsp_server::Request,
    server: &mut YkLangServer,
) -> Result<()> {
    conn.log_info(format!("Received {}", req.method));

    match req.method.as_str() {
        Completion::METHOD => completion(conn, req, server),
        _ => Ok(()),
    }
}

// pub fn did_change_text_document(
//     conn: &YkConnection,
//     req: lsp_server::Request,
//     server: &mut YkLangServer,
// ) -> Result<()> {

// }

fn cast<R>(
    req: lsp_server::Request,
) -> std::result::Result<(RequestId, R::Params), ExtractError<lsp_server::Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

pub fn completion(
    conn: &YkConnection,
    req: lsp_server::Request,
    server: &mut YkLangServer,
) -> Result<()> {
    let (req_id, params) = cast::<lsp::Completion>(req)?;

    let doc_pos = params.text_document_position;
    let Some(doc_line_index) = server.vfs.get_line_index(&doc_pos.text_document.uri) else {
        bail!("no document found for given uri")
    };

    let pos = from_lsp::offset(
        doc_line_index,
        doc_pos.position.line,
        doc_pos.position.character,
    )?;

    eprintln!("Position: {pos:?}");
    let q = {
        struct Anon;

        impl Anon {
            pub fn hello(self) {}
        }

        Anon
    };

    q.hello();

    conn.lsp_client
        .sender
        .send(lsp_server::Message::Response(Response::new_ok(
            req_id,
            CompletionResponse::Array(vec![
                CompletionItem {
                    label: "Je;;p".to_string(),
                    ..CompletionItem::default()
                },
                CompletionItem {
                    label: "SELECT".to_string(),
                    ..CompletionItem::default()
                },
            ]),
        )))
        .unwrap();
    Ok(())
}
