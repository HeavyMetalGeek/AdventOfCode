use anyhow::Result;
use std::fs::read_to_string;

pub enum Input {
    Test,
    Answer,
}

impl Input {
    pub fn value(&self) -> &str {
        match *self {
            Input::Test => "input/test.txt",
            Input::Answer => "input/stage1.txt",
        }
    }
    pub fn into_string(self) -> Result<String> {
        match self {
            Input::Test => Ok(read_to_string(Input::Test.value())?.parse()?),
            Input::Answer => Ok(read_to_string(Input::Answer.value())?.parse()?),
        }
    }
}
