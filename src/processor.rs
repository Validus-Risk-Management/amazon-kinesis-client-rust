use crate::checkpointer::Checkpointer;
use crate::messages::Record;
use eyre::Result;

pub trait Processor {
    fn initialize(&mut self, shard_id: &str);
    fn process_records(&mut self, data: &[Record], checkpoint: Checkpointer);
    fn lease_lost(&mut self);
    fn shard_ended(&mut self, checkpoint: Checkpointer);
    fn shutdown_requested(&mut self, checkpoint: Checkpointer);
}
