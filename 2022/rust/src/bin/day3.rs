use anyhow::{anyhow, Result};
use aoc::input::Input;
use std::{collections::HashSet, env, str::FromStr};

const DAY: usize = 3;

pub struct Pocket {
    pub content: Vec<char>,
    pub unique: HashSet<char>,
}

impl Pocket {
    pub fn new(content: Vec<char>) -> Self {
        let unique: HashSet<char> = HashSet::from_iter(content.clone());
        Self { content, unique }
    }
}

pub struct Rucksack {
    pub pockets: (Pocket, Pocket),
    pub common: char,
    pub priority: usize,
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let half = s.len() / 2;
        let pocket1 = Pocket::new(
            s.chars()
                .enumerate()
                .filter(|(i, _)| *i < half)
                .map(|t| t.1)
                .collect(),
        );
        let pocket2 = Pocket::new(
            s.chars()
                .enumerate()
                .filter(|(i, _)| *i >= half)
                .map(|t| t.1)
                .collect(),
        );
        let mut common = '0';
        for c in pocket1.unique.iter() {
            if let Some(ch) = pocket2.unique.get(c) {
                common = ch.to_owned();
                break;
            }
        }
        let priority = match common as u32 {
            x if (65..=90).contains(&x) => (x - 38) as usize,
            x if (97..=122).contains(&x) => (x - 96) as usize,
            _ => 0 as usize,
        };

        Ok(Self {
            pockets: (pocket1, pocket2),
            common,
            priority,
        })
    }
}

pub struct ElfGroup {
    pub badge: char,
    pub priority: usize,
}

impl ElfGroup {
    fn from_strings(s: &[&str]) -> Result<Self> {
        if s.len() < 3 {
            return Err(anyhow!("Elf group must be composed from 3 strings."));
        }
        let ruck1 = HashSet::from_iter(s[0].chars());
        let ruck2 = HashSet::from_iter(s[1].chars());
        let ruck3 = HashSet::from_iter(s[2].chars());
        let badge = Self::common_char(ruck1, ruck2, ruck3)?;
        let priority = match badge as u32 {
            x if (65..=90).contains(&x) => (x - 38) as usize,
            x if (97..=122).contains(&x) => (x - 96) as usize,
            _ => 0 as usize,
        };

        Ok(Self { badge, priority })
    }

    fn common_char(
        ruck1: HashSet<char>,
        ruck2: HashSet<char>,
        ruck3: HashSet<char>,
    ) -> Result<char> {
        ruck1
            .into_iter()
            .find(|c| ruck2.contains(c) && ruck3.contains(c))
            .ok_or_else(|| anyhow!("No badge char found"))
    }
}

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
