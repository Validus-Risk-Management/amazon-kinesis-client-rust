use crate::messages::Record;
use eyre::Result;

pub trait Processor {
    fn initialize(&mut self, shard_id: &str);
    fn process_records(
        &mut self,
        data: &[Record],
        checkpoint: impl Fn(Option<String>, Option<u64>) -> Result<()>,
    );
    fn lease_lost(&mut self);
    fn shard_ended(&mut self, checkpoint: impl Fn(Option<String>, Option<u64>) -> Result<()>);
    fn shutdown_requested(
        &mut self,
        checkpoint: impl Fn(Option<String>, Option<u64>) -> Result<()>,
    );
}
