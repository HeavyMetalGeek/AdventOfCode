use anyhow::Result;
use aoc::input::Input;
use aoc::station_assignment::SectionAssignment;
use std::env;

const DAY: usize = 4;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        match arg.as_str() {
            "stage1" | "1" => {
                println!("{}", stage1(Input::Answer)?);
                return Ok(());
            }
            "stage2" | "2" => {
                println!("{}", stage2(Input::Answer)?);
                return Ok(());
            }
            "test1" => {
                println!("{}", stage1(Input::Test)?);
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

fn stage1(input: Input) -> Result<usize> {
    let sections: Vec<SectionAssignment> = input
        .into_string(DAY)?
        .lines()
        .flat_map(str::parse::<SectionAssignment>)
        .collect();
    Ok(sections.iter().filter(|s| s.overlap1).count())
}

fn stage2(input: Input) -> Result<usize> {
    let sections: Vec<SectionAssignment> = input
        .into_string(DAY)?
        .lines()
        .flat_map(str::parse::<SectionAssignment>)
        .collect();
    Ok(sections.iter().filter(|s| s.overlap2).count())
}

#[test]
fn stage1_test() {
    let res = stage1(Input::Test).unwrap();
    assert_eq!(res, 2);
}

#[test]
fn stage2_test() {
    let res = stage2(Input::Test).unwrap();
    assert_eq!(res, 4);
}
