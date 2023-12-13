use ::anyhow::anyhow;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    pub dst_start: usize,
    pub src_start: usize,
    pub len: usize,
}

impl From<Vec<usize>> for Mapping {
    fn from(value: Vec<usize>) -> Self {
        Self {
            dst_start: value[0],
            src_start: value[1],
            len: value[2],
        }
    }
}

#[derive(Debug)]
pub struct Input {
    pub seeds: Vec<usize>,
    pub seed_to_soil: Vec<Mapping>,
    pub soil_to_fertilizer: Vec<Mapping>,
    pub fertilizer_to_water: Vec<Mapping>,
    pub water_to_light: Vec<Mapping>,
    pub light_to_temperature: Vec<Mapping>,
    pub temperature_to_humidity: Vec<Mapping>,
    pub humidity_to_location: Vec<Mapping>,
    pub parse_flag: i32,
    pub flag_map: BTreeMap<i32, &'static str>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            seeds: Vec::default(),
            seed_to_soil: Vec::default(),
            soil_to_fertilizer: Vec::default(),
            fertilizer_to_water: Vec::default(),
            water_to_light: Vec::default(),
            light_to_temperature: Vec::default(),
            temperature_to_humidity: Vec::default(),
            humidity_to_location: Vec::default(),
            parse_flag: i32::default(),
            flag_map: BTreeMap::from([
                (0b0000_0000, "seeds: "),
                (0b0000_0001, "seed-to-soil map: "),
                (0b0000_0010, "soil-to-fertilizer map: "),
                (0b0000_0100, "fertilizer-to-water map: "),
                (0b0000_1000, "water-to-light map: "),
                (0b0001_0000, "light_to_temperature map: "),
                (0b0010_0000, "temperature-to-humidity map: "),
                (0b0100_0000, "humidity-to-location map: "),
            ]),
        }
    }
}

impl Input {
    pub fn parse_line(&mut self, line: &str) -> anyhow::Result<()> {
        log::debug!("flag: {:#010b}, line: {}", self.parse_flag, line);
        match self.parse_flag {
            _ if self.parse_flag >> 7 == 1 => {
                self.parse_flag ^= 0b1000_0000;
            }
            _ if self.parse_flag >> 7 != 1 && line.is_empty() => {
                self.parse_flag <<= 1;
                if self.parse_flag == 0 {
                    self.parse_flag = 1;
                }
                self.parse_flag ^= 0b1000_0000;
            }
            0b0000_0000 => {
                match line
                    .trim()
                    .strip_prefix(self.flag_map.get(&self.parse_flag).unwrap())
                {
                    Some(rest) => {
                        self.seeds = rest
                            .trim()
                            .split(' ')
                            .filter_map(|v| v.parse::<usize>().ok())
                            .collect()
                    }
                    None => Err(anyhow!("Bad input for seeds"))?,
                };
            }
            0b0000_0001 => self.seed_to_soil.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            0b0000_0010 => self.soil_to_fertilizer.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            0b0000_0100 => self.fertilizer_to_water.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            0b0000_1000 => self.water_to_light.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            0b0001_0000 => self.light_to_temperature.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            0b0010_0000 => self.temperature_to_humidity.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            0b0100_0000 => self.humidity_to_location.push(Mapping::from(
                line.trim()
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>(),
            )),
            _ => Err(anyhow!("Invalid flag value with line: {}", line))?,
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AlmanacMap(BTreeMap<usize, usize>);

impl AlmanacMap {
    pub fn new(size: usize) -> Self {
        Self((0..size).map(|src| (src, src)).collect())
    }

    pub fn from_inputs(inputs: &[Vec<usize>]) -> Self {
        let mut map = BTreeMap::new();
        inputs.iter().for_each(|v| {
            (v[1]..v[1] + v[2]).enumerate().for_each(|(i, src)| {
                let _ = map.insert(src, v[0] + i);
            })
        });
        Self(map)
    }

    pub fn update(&mut self, dst_start: usize, src_start: usize, len: usize) {
        self.0.iter_mut().for_each(|(src, dst)| {
            if *src >= src_start && *src < src_start + len {
                *dst = dst_start + (src - src_start);
            }
        });
    }

    pub fn get(&self, index: usize) -> Option<&usize> {
        self.0.get(&index)
    }
}
