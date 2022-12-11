use std::collections::HashSet;
use std::str::FromStr;

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
