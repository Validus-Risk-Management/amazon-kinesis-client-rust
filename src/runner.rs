use crate::checkpointer::Checkpointer;

use eyre::Result;

use crate::messages::{
    parse_message, CheckpointWithErrorPayload, InitPayload, Message, ProcessRecordPayload,
};
use crate::processor::Processor;
use crate::reader::{InputReader, StdinReader};
use crate::responses::StatusResponse;
use crate::writer::{write_status, OutputWriter, StdoutWriter};

pub fn run(processor: &mut impl Processor<StdoutWriter, StdinReader>) {
    let mut reader = StdinReader::new();
    let mut writer = StdoutWriter::new();

    loop {
        tick(processor, &mut reader, &mut writer).unwrap();
    }
}

pub fn tick<W: OutputWriter, R: InputReader>(
    processor: &mut impl Processor<W, R>,
    input_reader: &mut R,
    output_writer: &mut W,
) -> Result<()> {
    let next = input_reader.next()?;
    let message = parse_message(&next)?;

    process_message(processor, &message, output_writer, input_reader);

    let status_message = StatusResponse::for_message(message);
    write_status(output_writer, status_message)?;

    Ok(())
}

pub(crate) fn process_message<W: OutputWriter, R: InputReader>(
    processor: &mut impl Processor<W, R>,
    message: &Message,
    output_writer: &mut W,
    input_reader: &mut R,
) {
    let mut checkpointer = Checkpointer::new(output_writer, input_reader);
    match message {
        Message::Initialize(InitPayload { shard_id }) => processor.initialize(shard_id),
        Message::ProcessRecords(ProcessRecordPayload { records }) => {
            processor.process_records(records, &mut checkpointer)
        }
        Message::LeaseLost => processor.lease_lost(),
        Message::ShardEnded(_) => processor.shard_ended(&mut checkpointer),
        Message::ShutdownRequested(_) => processor.shutdown_requested(&mut checkpointer),
        // This should only be sent in response to a checkpoint message sent to the daemon,
        // we should never receive it unexpectedly here
        Message::Checkpoint(CheckpointWithErrorPayload { checkpoint, error }) => panic!(
            "unexpected checkpoint: {:?}, error: {:?}",
            checkpoint, error
        ),
    }
}
