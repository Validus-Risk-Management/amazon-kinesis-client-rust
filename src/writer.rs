use std::io;
use std::io::{Stdout, Write};

use crate::responses::StatusResponse;
use eyre::Result;

pub(crate) fn write_status(writer: &mut impl OutputWriter, message: StatusResponse) -> Result<()> {
    let mut payload = serde_json::to_vec(&message)?;
    payload.push(b'\n');
    writer.write(payload.as_slice())?;

    Ok(())
}

pub trait OutputWriter {
    fn write(&mut self, payload: &[u8]) -> Result<()>;
}

pub struct StdoutWriter {
    out: Stdout,
}

impl StdoutWriter {
    pub(crate) fn new() -> Self {
        Self { out: io::stdout() }
    }
}

impl OutputWriter for StdoutWriter {
    fn write(&mut self, payload: &[u8]) -> Result<()> {
        self.out.write_all(payload)?;
        self.out.flush()?;

        Ok(())
    }
}
