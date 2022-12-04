use anyhow::Result;
use aoc::input::Input;
use std::env;

const DAY: usize = 1;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        match arg.as_str() {
            "stage1" | "1" => {
                println!("{}", stage1(Input::Answer)?.into_iter().max().unwrap_or(0));
                return Ok(());
            }
            "stage2" | "2" => {
                println!("{}", stage2(Input::Answer)?);
                return Ok(());
            }
            "test1" => {
                println!("{}", stage1(Input::Test)?.into_iter().max().unwrap_or(0));
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
    Ok(input
        .into_string(DAY)?
        .split("\n\n")
        .map(|s| s.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect())
}

fn stage2(input: Input) -> Result<usize> {
    let mut stage1 = stage1(input)?;
    stage1.sort_by(|a, b| b.cmp(a));
    Ok(stage1.into_iter().take(3).sum::<usize>())
}

#[test]
fn stage1_test() {
    let res = stage1(Input::Test).unwrap().into_iter().max().unwrap_or(0);
    assert_eq!(res, 24000);
}

#[test]
fn stage2_test() {
    let res = stage2(Input::Test).unwrap();
    assert_eq!(res, 45000);
}
