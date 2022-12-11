use anyhow::{anyhow, Result};
use std::collections::HashSet;

pub struct ElfGroup {
    pub badge: char,
    pub priority: usize,
}

impl ElfGroup {
    pub fn from_strings(s: &[&str]) -> Result<Self> {
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
