use kcl::reader::InputReader;
use std::collections::VecDeque;

#[derive(Default)]
pub struct MockReader {
    lines: VecDeque<String>,
}

impl MockReader {
    pub fn with_input(input: String) -> Self {
        let mut reader = Self::default();
        reader.add_input(input);

        reader
    }

    pub fn add_input(&mut self, input: String) {
        self.lines.push_back(input)
    }
}

impl InputReader for MockReader {
    fn next(&mut self) -> eyre::Result<String> {
        self.lines
            .pop_front()
            .ok_or(eyre::eyre!("no input to read"))
    }
}
