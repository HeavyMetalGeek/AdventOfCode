use anyhow::Result;
use std::fs::read_to_string;

pub enum Input {
    Test,
    Answer,
}

impl Input {
    pub fn value(&self, day: usize) -> String {
        match *self {
            Input::Test => format!("input/day{}/test.txt", day),
            Input::Answer => format!("input/day{}/stage1.txt", day),
        }
    }
    pub fn into_string(self, day: usize) -> Result<String> {
        match self {
            Input::Test => Ok(read_to_string(Input::Test.value(day))?.parse()?),
            Input::Answer => Ok(read_to_string(Input::Answer.value(day))?.parse()?),
        }
    }
}
