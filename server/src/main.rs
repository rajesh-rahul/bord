use capabilities::server_capabilities;
mod capabilities;
pub mod features;
mod from_lsp;
mod notifications;
mod requests;
mod responses;
mod utils;
mod vfs;
pub type Result<T> = anyhow::Result<T>;

use crossbeam::select;
use lsp_server::{Connection, Message};

#[derive(Default, Debug)]
struct YkLangServer {
    vfs: vfs::Vfs,
}

pub struct YkConnection {
    pub lsp_client: Connection,
    pub workers_recv: crossbeam::channel::Receiver<String>,
    pub workers_send: crossbeam::channel::Sender<String>,
}

fn main() -> anyhow::Result<()> {
    tracing::info!("Starting YkSQL Language Server");

    let (conn, io_threads) = Connection::stdio();

    let (workers_send, workers_recv) = crossbeam::channel::unbounded();

    let conn = YkConnection {
        lsp_client: conn,
        workers_recv,
        workers_send,
    };

    // Do initialization
    let server_capabilities = serde_json::to_value(server_capabilities())?;
    match conn.lsp_client.initialize(server_capabilities) {
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
        _ => {}
    }

    let server = YkLangServer::default();
    main_loop(conn, server)?;
    io_threads.join()?;

    tracing::info!("Shutting down YkSQL Language Server");
    Ok(())
}

pub enum Event {
    ClientLspMsg(Message),
    InternalMsg(String),
}

pub fn next_event(conn: &YkConnection) -> anyhow::Result<Event> {
    select! {
        recv(conn.lsp_client.receiver) -> msg => Ok(Event::ClientLspMsg(msg?)),
        recv(conn.workers_recv) -> msg => Ok(Event::InternalMsg(msg?)),
    }
}

fn main_loop(conn: YkConnection, mut server: YkLangServer) -> anyhow::Result<()> {
    loop {
        let event = next_event(&conn)?;

        match event {
            Event::ClientLspMsg(msg) => {
                if let Err(err) = handle_client_lsp_msg(&conn, msg, &mut server) {
                    eprintln!("Error handling message: {:?}", err);
                }
            }
            Event::InternalMsg(_) => {
                eprintln!("Received internal message");
            }
        }
    }
}

fn handle_client_lsp_msg(
    conn: &YkConnection,
    msg: Message,
    server: &mut YkLangServer,
) -> anyhow::Result<()> {
    match msg {
        Message::Request(req) => {
            if conn.lsp_client.handle_shutdown(&req)? {
                return Ok(());
            }
            requests::handle(&conn, req, server)?;
        }
        Message::Response(res) => responses::handle(&conn, res, server)?,
        Message::Notification(not) => notifications::handle(&conn, not, server)?,
    };

    Ok(())
}
