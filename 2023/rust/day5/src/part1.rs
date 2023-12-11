pub mod maps;

use aoc_lib::INPUT_DAY5;
use maps::{Input, Mapping};
use std::collections::BTreeMap;

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let mut input_maps = Input::default();
    for line in input.lines() {
        input_maps.parse_line(line)?;
    }
    Ok(input_maps)
}

fn get_min_loc(input: Input) -> anyhow::Result<usize> {
    let mut seed_to_soil = BTreeMap::new();
    for mapping in input.seed_to_soil {
        map_values(&input.seeds, &mapping, &mut seed_to_soil).unwrap();
    }
    log::debug!("Soil: {:?}", seed_to_soil);

    let mut soil_to_fertilizer = BTreeMap::new();
    let values: Vec<usize> = seed_to_soil.into_values().collect();
    for mapping in input.soil_to_fertilizer {
        map_values(&values, &mapping, &mut soil_to_fertilizer).unwrap();
    }
    log::debug!("Fert: {:?}", soil_to_fertilizer);

    let mut fertilizer_to_water = BTreeMap::new();
    let values: Vec<usize> = soil_to_fertilizer.into_values().collect();
    for mapping in input.fertilizer_to_water {
        map_values(&values, &mapping, &mut fertilizer_to_water).unwrap();
    }
    log::debug!("Water: {:?}", fertilizer_to_water);

    let mut water_to_light = BTreeMap::new();
    let values: Vec<usize> = fertilizer_to_water.into_values().collect();
    for mapping in input.water_to_light {
        map_values(&values, &mapping, &mut water_to_light).unwrap();
    }
    log::debug!("Light: {:?}", water_to_light);

    let mut light_to_temperature = BTreeMap::new();
    let values: Vec<usize> = water_to_light.into_values().collect();
    for mapping in input.light_to_temperature {
        map_values(&values, &mapping, &mut light_to_temperature).unwrap();
    }
    log::debug!("Temp: {:?}", light_to_temperature);

    let mut temperature_to_humidity = BTreeMap::new();
    let values: Vec<usize> = light_to_temperature.into_values().collect();
    for mapping in input.temperature_to_humidity {
        map_values(&values, &mapping, &mut temperature_to_humidity).unwrap();
    }
    log::debug!("Humidities: {:?}", temperature_to_humidity);

    let mut humidity_to_location = BTreeMap::new();
    let values: Vec<usize> = temperature_to_humidity.into_values().collect();
    for mapping in input.humidity_to_location {
        map_values(&values, &mapping, &mut humidity_to_location).unwrap();
    }
    log::debug!("Locations: {:?}", humidity_to_location);

    let min_loc: usize = humidity_to_location
        .values()
        .cloned()
        .fold(usize::MAX, |loc_min, loc| loc.min(loc_min));

    Ok(min_loc)
}

fn map_values(
    seeds: &[usize],
    mapping: &Mapping,
    map: &mut BTreeMap<usize, usize>,
) -> anyhow::Result<()> {
    log::debug!("Mapping: {:?}", mapping);
    for seed in seeds.iter() {
        let seed = *seed;
        if seed >= mapping.src_start && seed < mapping.src_start + mapping.len {
            let dst = mapping.dst_start + (seed - mapping.src_start);
            log::debug!("Inserting: {}:{}", seed, dst);
            if let Some(v) = map.insert(seed, dst) {
                log::debug!("Overlapping key: {:?}", v);
            }
        }
        map.entry(seed).or_insert_with(|| {
            log::debug!("Inserting: {}:{}", seed, seed);
            seed
        });
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let inputs = parse_input(INPUT_DAY5)?;
    let min_loc = get_min_loc(inputs)?;
    println!("Result: {}", min_loc);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day5part1_solution_test() {
        init_logger();
        let inputs = parse_input(INPUT_DAY5).unwrap();
        assert_eq!(get_min_loc(inputs).unwrap(), 331445006);
    }

    #[test]
    fn day5part1_test() {
        init_logger();
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        let inputs = parse_input(input).unwrap();
        assert_eq!(get_min_loc(inputs).unwrap(), 35);
    }
}
