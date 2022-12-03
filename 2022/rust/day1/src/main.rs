use anyhow::Result;
use std::env;
use std::fs::read_to_string;

enum Input {
    Test,
    Answer,
}

impl Input {
    fn value(&self) -> &str {
        match *self {
            Input::Test => "input/test.txt",
            Input::Answer => "input/stage1.txt",
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        match arg.as_str() {
            "stage1" | "1" => {
                println!("{}", stage1(Input::Answer)?.iter().max().unwrap_or(&0));
                return Ok(());
            }
            "stage2" | "2" => {
                println!("{}", stage2(Input::Answer)?);
                return Ok(());
            }
            "test1" => {
                println!("{}", stage1(Input::Test)?.iter().max().unwrap_or(&0));
                return Ok(());
            }
            "test2" => {
                println!("{}", stage2(Input::Test)?);
                return Ok(());
            }
            _ => continue,
        }
    }
    println!("You must choose a run mode");
    Ok(())
}

fn stage1(input: Input) -> Result<Vec<usize>> {
    let txt: String = match input {
        Input::Test => read_to_string(Input::Test.value())?.parse()?,
        Input::Answer => read_to_string(Input::Answer.value())?.parse()?,
    };
    Ok(txt
        .split("\n\n")
        .map(|s| s.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect())
}

fn stage2(input: Input) -> Result<usize> {
    let mut stage1 = stage1(input)?;
    stage1.sort_by(|a, b| b.partial_cmp(a).expect("Sort failed"));
    Ok(stage1.into_iter().take(3).sum::<usize>())
}
