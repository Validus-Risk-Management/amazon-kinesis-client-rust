use serde::Serialize;

use crate::messages::Message;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StatusResponse {
    action: String,
    response_for: String,
}

impl StatusResponse {
    pub fn for_message(message: Message) -> Self {
        let response_for = match message {
            Message::Initialize(_) => "initialize",
            Message::ProcessRecords(_) => "processRecords",
            Message::Checkpoint(_) => "checkpoint",
            Message::LeaseLost => "leaseLost",
            Message::ShardEnded(_) => "shardEnded",
            Message::ShutdownRequested(_) => "shutdownRequested",
        }
        .to_string();

        Self {
            action: "status".to_string(),
            response_for,
        }
    }
}
