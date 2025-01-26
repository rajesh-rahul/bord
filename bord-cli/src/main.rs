use bord_server::BordLangServer;
use tower_lsp::{LspService, Server};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .finish();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) =  LspService::new(|client| BordLangServer::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;
}
