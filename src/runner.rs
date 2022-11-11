use eyre::Result;

use crate::messages::{parse_message, InitPayload, Message, ProcessRecordPayload};
use crate::processor::Processor;
use crate::reader::{InputReader, StdinReader};
use crate::responses::StatusResponse;
use crate::writer::{write_status, OutputWriter, StdoutWriter};

pub fn run(processor: &mut impl Processor) {
    let mut reader = StdinReader::new();
    let mut writer = StdoutWriter::new();

    loop {
        tick(processor, &mut reader, &mut writer).unwrap();
    }
}

pub fn tick(
    processor: &mut impl Processor,
    input_reader: &mut impl InputReader,
    output_writer: &mut impl OutputWriter,
) -> Result<()> {
    let next = input_reader.next()?;
    let message = parse_message(&next)?;

    process_message(processor, &message);

    let status_message = StatusResponse::for_message(message);
    write_status(output_writer, status_message)?;

    Ok(())
}

fn process_message(processor: &mut impl Processor, message: &Message) {
    match message {
        Message::Initialize(InitPayload { shard_id }) => processor.initialize(shard_id),
        Message::ProcessRecords(ProcessRecordPayload { records }) => {
            processor.process_records(records)
        }
        Message::LeaseLost => processor.lease_lost(),
        Message::ShardEnded(_) => processor.shard_ended(),
        Message::ShutdownRequested(_) => processor.shutdown_requested(),
        // This should only be sent in response to a checkpoint message sent to the daemon,
        // we should never receive it unexpectedly here
        Message::Checkpoint(_) => panic!("unexpected checkpointing"),
    }
}
