use crate::messages::Record;

pub trait Processor {
    fn initialize(&mut self, shard_id: &str);
    fn process_records(&mut self, data: &[Record]);
}
