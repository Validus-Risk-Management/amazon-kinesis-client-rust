use kcl::checkpointer::Checkpointer;
use kcl::reader::StdinReader;
use kcl::writer::StdoutWriter;
use kcl::{run, Processor, Record};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyPayload {
    event_field: String,
}

struct MyConsumer;

impl Processor<StdoutWriter, StdinReader> for MyConsumer {
    fn initialize(&mut self, _shard_id: &str) {}

    fn process_records(
        &mut self,
        data: &[Record],
        checkpointer: &mut Checkpointer<StdoutWriter, StdinReader>,
    ) {
        for record in data {
            match record.json::<MyPayload>() {
                Ok(data) => println!("{:?}", data.event_field),
                Err(e) => println!("{:?}", e),
            }
        }
        checkpointer
            .checkpoint(None, None)
            .expect("Checkpoint to succeed.");
    }
    fn lease_lost(&mut self) {}
    fn shard_ended(&mut self, checkpointer: &mut Checkpointer<StdoutWriter, StdinReader>) {
        checkpointer
            .checkpoint(None, None)
            .expect("Checkpoint to succeed.");
    }
    fn shutdown_requested(&mut self, checkpointer: &mut Checkpointer<StdoutWriter, StdinReader>) {
        checkpointer
            .checkpoint(None, None)
            .expect("Checkpoint to succeed.");
    }
}

fn main() {
    run(&mut MyConsumer {});
}
