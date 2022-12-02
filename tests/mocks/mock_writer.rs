use kcl::writer::OutputWriter;

#[derive(Default)]
pub struct MockWriter {
    pub outputs: Vec<String>,
}

impl OutputWriter for MockWriter {
    fn write(&mut self, payload: &[u8]) -> eyre::Result<()> {
        let output = std::str::from_utf8(payload)?;
        self.outputs.push(output.to_owned());

        Ok(())
    }
}
