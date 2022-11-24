use crate::checkpointer::Checkpointer;
use crate::messages::Record;
use crate::writer::OutputWriter;
use eyre::Result;

pub trait Processor<T: OutputWriter> {
    fn initialize(&mut self, shard_id: &str);
    fn process_records(&mut self, data: &[Record], checkpoint: &mut Checkpointer<T>);
    fn lease_lost(&mut self);
    fn shard_ended(&mut self, checkpoint: &mut Checkpointer<T>);
    fn shutdown_requested(&mut self, checkpoint: &mut Checkpointer<T>);
}
