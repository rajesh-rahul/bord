use lsp_types::{
    notification::{LogMessage, Notification},
    LogMessageParams, MessageType,
};

use crate::YkConnection;

pub trait ConnectionExt {
    fn log_info(&self, message: impl Into<String>);
}

impl ConnectionExt for YkConnection {
    fn log_info(&self, message: impl Into<String>) {
        self.lsp_client.sender.send(
            lsp_server::Notification::new(
                LogMessage::METHOD.into(),
                LogMessageParams {
                    typ: MessageType::INFO,
                    message: message.into(),
                },
            )
            .into(),
        );
    }
}
