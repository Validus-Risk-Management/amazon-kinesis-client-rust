use amazon_kinesis_client::writer::OutputWriter;

pub struct MockWriter {
    pub outputs: Vec<String>,
}

impl MockWriter {
    pub(crate) fn new() -> Self {
        Self { outputs: vec![] }
    }
}

impl OutputWriter for MockWriter {
    fn write(&mut self, payload: &[u8]) -> eyre::Result<()> {
        let output = std::str::from_utf8(payload)?;
        self.outputs.push(output.to_owned());

        Ok(())
    }
}
