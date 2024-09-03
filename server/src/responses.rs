use crate::utils::ConnectionExt;
use crate::YkConnection;
use crate::YkLangServer;
use crate::Result;
use lsp_server::ExtractError;
use lsp_server::Request;
use lsp_server::RequestId;
use lsp_server::Response;

pub fn handle(conn: &YkConnection, res: Response, server: &mut YkLangServer) -> Result<()> {
    conn.log_info(format!("Received {:?}", res));

    match res.id {
        _ => Ok(()),
    }
}

fn cast<R>(req: Request) -> std::result::Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
