use std::ops::ControlFlow;

use async_lsp::lsp_types::notification::Notification;
use async_lsp::lsp_types::{notification, InitializeParams, InitializedParams};
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::LanguageServer;
use futures::AsyncReadExt;
use serde_json::Value;
use tokio_util::compat::TokioAsyncReadCompatExt;
use tower::ServiceBuilder;

const MEMORY_CHANNEL_SIZE: usize = 64 << 10; // 64KiB
struct ClientState;

#[tokio::test(flavor = "current_thread")]
async fn run_incr_parser_usages() {
    let entries = std::fs::read_dir("tests/logs")
        .unwrap()
        .flatten()
        .filter(|it| it.path().extension().is_some_and(|it| it == "log"))
        .collect::<Vec<_>>();

    assert!(entries.len() == 9);

    for entry in entries {
        let traces = get_language_client_trace(entry.path());
        mock_server_and_client(traces).await;
    }
}

fn get_language_client_trace(file_name: std::path::PathBuf) -> Vec<serde_json::Value> {
    eprintln!("{file_name:?}");
    let file_data = std::fs::read_to_string(file_name).unwrap();

    file_data
        .lines()
        .filter_map(|it| it.split("incoming msg=").nth(1))
        .map(|it| serde_json::from_str(it).unwrap())
        .collect()
}

// NOTE: Following code is adapted from testing example code from async-lsp
async fn mock_server_and_client(traces: Vec<Value>) {
    // The server with handlers.
    let (server_main, _) = async_lsp::MainLoop::new_server(|client| {
        let router = bord_server::router(client);

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .service(router)
    });

    // The client with handlers.
    let (client_main, mut server) = async_lsp::MainLoop::new_client(|_server| {
        let mut router = Router::new(ClientState);
        router
            // Apparently this handling this notification is mandatory
            .notification::<notification::PublishDiagnostics>(|_, _| ControlFlow::Continue(()));

        ServiceBuilder::new().service(router)
    });

    // Wire up a loopback channel between the server and the client.
    let (server_stream, client_stream) = tokio::io::duplex(MEMORY_CHANNEL_SIZE);
    let (server_rx, server_tx) = server_stream.compat().split();
    let server_main = tokio::spawn(async move {
        server_main
            .run_buffered(server_rx, server_tx)
            .await
            .unwrap();
    });

    let (client_rx, client_tx) = client_stream.compat().split();
    let client_main = tokio::spawn(async move {
        let err = client_main
            .run_buffered(client_rx, client_tx)
            .await
            .unwrap_err();
        assert!(
            matches!(err, async_lsp::Error::Eof),
            "should fail due to EOF: {err}"
        );
    });

    // Send requests to the server on behalf of the client, via `ServerSocket`. It interacts with
    // the client main loop to finalize and send the request through the channel.
    server
        .initialize(InitializeParams::default())
        .await
        .unwrap();

    server.initialized(InitializedParams {}).unwrap();

    for rpc_message in traces {
        match rpc_message["method"].as_str().unwrap() {
            notification::DidOpenTextDocument::METHOD => {
                let params = serde_json::from_value(rpc_message["params"].clone()).unwrap();
                server
                    .notify::<notification::DidOpenTextDocument>(params)
                    .unwrap();
            }
            notification::DidChangeTextDocument::METHOD => {
                let params = serde_json::from_value(rpc_message["params"].clone()).unwrap();
                server
                    .notify::<notification::DidChangeTextDocument>(params)
                    .unwrap();
            }
            _ => {}
        }
    }

    // Shutdown the server.
    server.shutdown(()).await.unwrap();
    server.exit(()).unwrap();

    // Both main loop should be shutdown.
    server_main.await.expect("no panic");
    client_main.await.expect("no panic");
}
