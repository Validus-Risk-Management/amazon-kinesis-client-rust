use crate::checkpointer::Checkpointer;
use crate::messages::Record;
use crate::reader::InputReader;
use crate::writer::OutputWriter;

pub trait Processor<W: OutputWriter, R: InputReader> {
    fn initialize(&mut self, shard_id: &str);
    fn process_records(&mut self, data: &[Record], checkpoint: &mut Checkpointer<W, R>);
    fn lease_lost(&mut self);
    fn shard_ended(&mut self, checkpoint: &mut Checkpointer<W, R>);
    fn shutdown_requested(&mut self, checkpoint: &mut Checkpointer<W, R>);
}
