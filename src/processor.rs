use crate::messages::{parse_message, Message};
use serde::Serialize;
use std::io;
use std::io::Write;

fn read_next() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("{input}");
    }

    input
}

fn write_status(message: StatusMessage) {
    let mut out = io::stdout();
    let mut payload = serde_json::to_vec(&message).unwrap();
    payload.push(b'\n');
    out.write_all(payload.as_slice()).unwrap();
    out.flush().unwrap();
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusMessage {
    action: String,
    response_for: String,
}

impl StatusMessage {
    fn new_for_message(message: Message) -> Self {
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

pub fn run() {
    loop {
        let next = read_next();
        let message = parse_message(&next).unwrap();
        let status_message = StatusMessage::new_for_message(message);

        write_status(status_message);
    }
}
