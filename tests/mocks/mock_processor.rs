use crate::mocks::mock_reader::MockReader;
use crate::mocks::mock_writer::MockWriter;
use amazon_kinesis_client::checkpointer::Checkpointer;
use amazon_kinesis_client::{Processor, Record};

pub struct MockProcessor {
    pub shard: Option<String>,
    pub records: Vec<Record>,
    pub lease_lost: bool,
    pub shard_ended: bool,
    pub shutdown_requested: bool,
}

impl MockProcessor {
    pub fn new() -> Self {
        Self {
            shard: None,
            records: vec![],
            lease_lost: false,
            shard_ended: false,
            shutdown_requested: false,
        }
    }
}

impl Processor<MockWriter, MockReader> for MockProcessor {
    fn initialize(&mut self, shard_id: &str) {
        self.shard = Some(shard_id.to_owned())
    }

    fn process_records(
        &mut self,
        data: &[Record],
        _checkpointer: &mut Checkpointer<MockWriter, MockReader>,
    ) {
        for record in data {
            self.records.push((*record).clone())
        }
    }

    fn lease_lost(&mut self) {
        self.lease_lost = true;
    }
    fn shard_ended(&mut self, _checkpointer: &mut Checkpointer<MockWriter, MockReader>) {
        self.shard_ended = true;
    }
    fn shutdown_requested(&mut self, _checkpointer: &mut Checkpointer<MockWriter, MockReader>) {
        self.shutdown_requested = true;
    }
}
