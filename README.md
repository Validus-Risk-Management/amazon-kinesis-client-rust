# Amazon Kinesis Client Library for Rust

This package provides a Rust interface to the Amazon Kinesis Client Library (KCL) MultiLangDaemon,
which is part of the [Amazon KCL for Java][kinesis-github].

This interface manages the interaction with the MultiLangDaemon so that developers can focus on
implementing their record processor executable.

There is a provided Docker image that sets up the correct JARs using the [Amazon KCL for Python][kinesis-python].

A settings file is also required for the MultiLangDaemon to correctly set up your processor.
A sample of this can be found in the [examples](./examples/sample.properties)

## Docker

An example consumer of this Docker Image would be:

**Compile with al2 because amazoncoretto uses al2**

```dockerfile
FROM amazonlinux:2 as builder
RUN amazon-linux-extras install rust1
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

Additional configuration can be found [here][kcl-cli-params]


[amazon-kcl]: http://docs.aws.amazon.com/kinesis/latest/dev/kinesis-record-processor-app.html
[kinesis-github]: https://github.com/awslabs/amazon-kinesis-client
[kinesis-python]: https://github.com/awslabs/amazon-kinesis-client-python
[kcl-cli-params]: https://github.com/awslabs/amazon-kinesis-client-python/blob/v2.0.6/samples/amazon_kclpy_helper.py
