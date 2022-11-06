use std::io;

use crate::messages::{parse_message, InitPayload, Message, ProcessRecordPayload};
use crate::processor::Processor;
use crate::responses::acknowledge_message;

fn read_next() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("{input}");
    }

    input
}

pub fn run(processor: &mut impl Processor) {
    loop {
        let next = read_next();
        let message = parse_message(&next).unwrap();
        match &message {
            Message::Initialize(InitPayload { shard_id }) => processor.initialize(shard_id),
            Message::ProcessRecords(ProcessRecordPayload { records }) => {
                processor.process_records(records)
            }
            Message::LeaseLost => {}
            Message::ShardEnded(_) => {}
            Message::ShutdownRequested(_) => {}
            // This should only be sent in response to a checkpoint message sent to the daemon,
            // we should never receive it unexpectedly here
            Message::Checkpoint(_) => panic!("unexpected checkpointing"),
        }

        acknowledge_message(message);
    }
}
