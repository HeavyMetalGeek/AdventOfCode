use anyhow::Result;
use aoc::elfgroup::ElfGroup;
use aoc::input::Input;
use aoc::rucksack::Rucksack;
use std::env;

const DAY: usize = 3;

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
    let rucks: Vec<Rucksack> = input
        .into_string(DAY)?
        .lines()
        .flat_map(str::parse::<Rucksack>)
        .collect();
    Ok(rucks.iter().map(|r| r.priority).sum())
}

fn stage2(input: Input) -> Result<usize> {
    let rucks: Vec<ElfGroup> = input
        .into_string(DAY)?
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|v| ElfGroup::from_strings(v).unwrap())
        .collect();
    Ok(rucks.iter().map(|r| r.priority).sum())
}

#[test]
fn stage1_test() {
    let res = stage1(Input::Test).unwrap();
    assert_eq!(res, 157);
}

#[test]
fn stage2_test() {
    let res = stage2(Input::Test).unwrap();
    assert_eq!(res, 70);
}
