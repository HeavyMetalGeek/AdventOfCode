mod input;
mod play;

use anyhow::Result;
use input::Input;
use play::{Play, PlayResult};
use std::env;

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
    Ok(input.into_string()?.lines().flat_map(score1).sum::<usize>())
}

fn score1(guide_text: &str) -> Result<usize> {
    let round: Vec<&str> = guide_text.split(' ').take(2).to_owned().collect();
    let opp_play = Play::get(round[0])?;
    let my_play = Play::get(round[1])?;
    let result = PlayResult::get(&opp_play, &my_play);
    Ok(result.pts() + my_play.pts())
}

fn stage2(input: Input) -> Result<usize> {
    Ok(input.into_string()?.lines().flat_map(score2).sum::<usize>())
}

fn score2(guide_text: &str) -> Result<usize> {
    let round: Vec<&str> = guide_text.split(' ').take(2).to_owned().collect();
    let opp_play = Play::get(round[0])?;
    let result = PlayResult::from_code(round[1])?;
    let my_play = match result {
        PlayResult::Win => opp_play.win_agaist(),
        PlayResult::Lose => opp_play.lose_agaist(),
        PlayResult::Draw => opp_play,
    };
    Ok(result.pts() + my_play.pts())
}

#[test]
fn stage1_test() {
    let res = stage1(Input::Test).unwrap();
    assert_eq!(res, 15);
}

#[test]
fn stage2_test() {
    let res = stage2(Input::Test).unwrap();
    assert_eq!(res, 12);
}
