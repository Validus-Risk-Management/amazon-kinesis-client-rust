use std::io;
use std::io::Write;

use eyre::Result;
use serde::Serialize;

use crate::messages::Message;

pub(crate) fn acknowledge_message(message: Message) -> Result<()> {
    let status_message = StatusResponse::for_message(message);
    write_status(status_message)?;

    Ok(())
}

fn write_status(message: StatusResponse) -> Result<()> {
    let mut out = io::stdout();
    let mut payload = serde_json::to_vec(&message)?;
    payload.push(b'\n');
    out.write_all(payload.as_slice())?;
    out.flush()?;

    Ok(())
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
