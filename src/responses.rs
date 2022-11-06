use crate::messages::Message;
use std::io;
use std::io::Write;

use serde::Serialize;

pub(crate) fn acknowledge_message(message: Message) {
    let status_message = StatusResponse::for_message(message);
    write_status(status_message);
}

fn write_status(message: StatusResponse) {
    let mut out = io::stdout();
    let mut payload = serde_json::to_vec(&message).unwrap();
    payload.push(b'\n');
    out.write_all(payload.as_slice()).unwrap();
    out.flush().unwrap();
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusResponse {
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
