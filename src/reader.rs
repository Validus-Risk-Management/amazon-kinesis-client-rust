use std::io;

use eyre::Result;

pub trait InputReader {
    fn next(&mut self) -> Result<String>;
}

pub struct StdinReader {
    stdin: io::Stdin,
}

impl StdinReader {
    pub(crate) fn new() -> Self {
        Self { stdin: io::stdin() }
    }
}

impl InputReader for StdinReader {
    fn next(&mut self) -> Result<String> {
        let mut input = String::new();
        self.stdin.read_line(&mut input)?;

        Ok(input)
    }
}
