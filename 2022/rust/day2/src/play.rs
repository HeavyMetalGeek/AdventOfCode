use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn get(play: &str) -> Result<Play> {
        match play {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(anyhow!("Failed to match play")),
        }
    }
    pub fn pts(&self) -> usize {
        match *self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
    pub fn lose_agaist(&self) -> Play {
        match *self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }
    pub fn win_agaist(&self) -> Play {
        match *self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PlayResult {
    Win,
    Lose,
    Draw,
}

impl PlayResult {
    pub fn get(opponent: &Play, me: &Play) -> PlayResult {
        match (opponent, me) {
            (Play::Rock, Play::Rock) => PlayResult::Draw,
            (Play::Rock, Play::Paper) => PlayResult::Win,
            (Play::Rock, Play::Scissors) => PlayResult::Lose,
            (Play::Paper, Play::Rock) => PlayResult::Lose,
            (Play::Paper, Play::Paper) => PlayResult::Draw,
            (Play::Paper, Play::Scissors) => PlayResult::Win,
            (Play::Scissors, Play::Rock) => PlayResult::Win,
            (Play::Scissors, Play::Paper) => PlayResult::Lose,
            (Play::Scissors, Play::Scissors) => PlayResult::Draw,
        }
    }
    pub fn pts(&self) -> usize {
        match *self {
            PlayResult::Win => 6,
            PlayResult::Draw => 3,
            PlayResult::Lose => 0,
        }
    }
    pub fn from_code(code: &str) -> Result<Self> {
        match code {
            "X" => Ok(PlayResult::Lose),
            "Y" => Ok(PlayResult::Draw),
            "Z" => Ok(PlayResult::Win),
            _ => Err(anyhow!("Couldn't match code to play result")),
        }
    }
}
