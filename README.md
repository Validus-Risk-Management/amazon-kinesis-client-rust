# Amazon Kinesis Client Library for Rust
[![crates-badge]](https://crates.io/crates/kcl)
[![docs-badge]](https://docs.rs/kcl)
[![Crates.io](https://img.shields.io/crates/l/kcl)](LICENSE)

This package provides a Rust interface to the Amazon Kinesis Client Library (KCL) MultiLangDaemon,
which is part of the [Amazon KCL for Java][kinesis-github].

This interface manages the interaction with the MultiLangDaemon so that developers can focus on
implementing their record processor executable.

There is a provided Docker image that sets up the correct JARs using the [Amazon KCL for Python][kinesis-python].

A settings file is also required for the MultiLangDaemon to correctly set up your processor.
A sample of this can be found in the [examples][example-properties].


## Basic Usage

A more complete example can be found in the [example][example-consumer]

```rust no_run
use kcl::checkpointer::Checkpointer;
use kcl::reader::StdinReader;
use kcl::writer::StdoutWriter;
use kcl::{run, Processor, Record};
use serde::Deserialize;

#[derive(Deserialize)]
struct DummyPayload;
struct BaseApp;

impl Processor<StdoutWriter, StdinReader> for BaseApp {
    fn initialize(&mut self, _shard_id: &str) {}

    fn process_records(
        &mut self,
        data: &[Record],
        _checkpointer: &mut Checkpointer<StdoutWriter, StdinReader>,
    ) {
        for record in data {
            match record.json::<DummyPayload>() {
                Ok(data) => {}
                Err(e) => {}
            }
        }
    }
    fn lease_lost(&mut self) {}
    fn shard_ended(&mut self, _checkpointer: &mut Checkpointer<StdoutWriter, StdinReader>) {}
    fn shutdown_requested(&mut self, _checkpointer: &mut Checkpointer<StdoutWriter, StdinReader>) {}
}

fn main() {
    run(&mut BaseApp {});
}

```


## Docker

An example consumer of this Docker Image would be:

**Compile with al2 because amazoncoretto uses al2**

```dockerfile
FROM amazonlinux:2 as builder
RUN yum update -y && yum install -y gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY . .
RUN cargo build --release

FROM ghcr.io/validus-risk-management/amazon-kinesis-client-rust:latest as runner
COPY my-configs/app.properties app.properties
COPY --from=builder target/release/my-app target/release/my-app
```

The default entrypoint should meet most requirements:

```dockerfile
CMD ["java", "-cp", "/usr/local/lib/jars/*", "software.amazon.kinesis.multilang.MultiLangDaemon", "--properties-file", "app.properties"]
```

Additional configuration can be found [here][kcl-cli-params].


[amazon-kcl]: http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-record-processor-app.html
[kinesis-github]: https://github.com/awslabs/amazon-kinesis-client
[kinesis-python]: https://github.com/awslabs/amazon-kinesis-client-python
[kcl-cli-params]: https://github.com/awslabs/amazon-kinesis-client-python/blob/v2.0.6/samples/amazon_kclpy_helper.py
[example-properties]: https://github.com/Validus-Risk-Management/amazon-kinesis-client-rust/blob/main/examples/sample.properties
[example-consumer]: https://github.com/Validus-Risk-Management/amazon-kinesis-client-rust/blob/main/examples/example_consumer/main.rs
[crates-badge]: https://img.shields.io/crates/v/kcl.svg
[docs-badge]: https://docs.rs/kcl/badge.svg