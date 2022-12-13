use anyhow::{anyhow, Result};
use std::str::FromStr;

pub struct SectionAssignment {
    pub section1: (usize, usize),
    pub section2: (usize, usize),
    pub overlap1: bool,
    pub overlap2: bool,
}

impl FromStr for SectionAssignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split(',');
        let mut section1 = sections
            .next()
            .ok_or_else(|| anyhow!("Failed to get section 1"))?
            .split('-')
            .flat_map(str::parse::<usize>)
            .take(2);
        let section1_lo = section1
            .next()
            .ok_or_else(|| anyhow!("Failed to get section 1 lo value"))?;
        let section1_hi = section1
            .next()
            .ok_or_else(|| anyhow!("Failed to get section 1 hi value"))?;
        let mut section2 = sections
            .next()
            .ok_or_else(|| anyhow!("Failed to get section 2"))?
            .split('-')
            .flat_map(str::parse::<usize>)
            .take(2);
        let section2_lo = section2
            .next()
            .ok_or_else(|| anyhow!("Failed to get section 2 lo value"))?;
        let section2_hi = section2
            .next()
            .ok_or_else(|| anyhow!("Failed to get section 2 hi value"))?;
        let overlap1 = (section1_lo <= section2_lo && section1_hi >= section2_hi)
            || (section1_lo >= section2_lo && section1_hi <= section2_hi);
        let overlap2 = (section1_lo >= section2_lo && section1_lo <= section2_hi)
            || (section2_lo >= section1_lo && section2_lo <= section1_hi)
            || (section1_hi <= section2_hi && section1_hi >= section2_lo)
            || (section2_hi <= section1_hi && section2_hi >= section1_lo);
        Ok(Self {
            section1: (section1_lo, section1_hi),
            section2: (section2_lo, section2_hi),
            overlap1,
            overlap2,
        })
    }
}
