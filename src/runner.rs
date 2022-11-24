use crate::checkpointer::Checkpointer;
use std::borrow::BorrowMut;

use eyre::Result;

use crate::messages::{
    parse_message, CheckpointWithErrorPayload, InitPayload, Message, ProcessRecordPayload,
};
use crate::processor::Processor;
use crate::reader::{InputReader, StdinReader};
use crate::responses::StatusResponse;
use crate::writer::{write_status, OutputWriter, StdoutWriter};

pub fn run(processor: &mut impl Processor<StdoutWriter>) {
    let mut reader = StdinReader::new();
    let mut writer = &mut StdoutWriter::new();
    let mut checkpointer = Checkpointer::new(writer);

    loop {
        tick(processor, &mut reader, writer, &mut checkpointer).unwrap();
    }
}

pub fn tick<T: OutputWriter>(
    processor: &mut impl Processor<T>,
    input_reader: &mut impl InputReader,
    output_writer: &mut impl OutputWriter,
    checkpointer: &mut Checkpointer<T>,
) -> Result<()> {
    let next = input_reader.next()?;
    let message = parse_message(&next)?;

    process_message(processor, &message, checkpointer);

    let status_message = StatusResponse::for_message(message);
    write_status(output_writer, status_message)?;

    Ok(())
}

fn process_message<T: OutputWriter>(
    processor: &mut impl Processor<T>,
    message: &Message,
    checkpointer: &mut Checkpointer<T>,
) {
    match message {
        Message::Initialize(InitPayload { shard_id }) => processor.initialize(shard_id),
        Message::ProcessRecords(ProcessRecordPayload { records }) => {
            processor.process_records(records, checkpointer)
        }
        Message::LeaseLost => processor.lease_lost(),
        Message::ShardEnded(_) => processor.shard_ended(checkpointer),
        Message::ShutdownRequested(_) => processor.shutdown_requested(checkpointer),
        // This should only be sent in response to a checkpoint message sent to the daemon,
        // we should never receive it unexpectedly here
        Message::Checkpoint(CheckpointWithErrorPayload { checkpoint, error }) => panic!(
            "unexpected checkpoint: {:?}, error: {:?}",
            checkpoint, error
        ),
    }
}
