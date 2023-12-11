pub mod maps;

use aoc_lib::INPUT_DAY5;
use maps::{Input, Mapping};
use rayon::prelude::*;

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let mut input_maps = Input::default();
    for line in input.lines() {
        input_maps.parse_line(line)?;
    }
    Ok(input_maps)
}

fn map_seed(seed: usize, mappings: &[Mapping]) -> anyhow::Result<usize> {
    let dst: Vec<usize> = mappings
        .iter()
        .filter_map(|mapping| {
            if seed >= mapping.src_start && seed < mapping.src_start + mapping.len {
                let dst = mapping.dst_start + (seed - mapping.src_start);
                Some(dst)
            } else {
                None
            }
        })
        .collect();
    if dst.is_empty() {
        Ok(seed)
    } else if dst.len() > 1 {
        Err(anyhow::anyhow!("Multiple mappings found for seed!"))?
    } else {
        Ok(dst[0])
    }
}

fn get_seed_loc(input: &Input, seed: usize) -> anyhow::Result<usize> {
    let seed_to_soil = map_seed(seed, &input.seed_to_soil)?;
    let soil_to_fertilizer = map_seed(seed_to_soil, &input.soil_to_fertilizer)?;
    let fertilizer_to_water = map_seed(soil_to_fertilizer, &input.fertilizer_to_water)?;
    let water_to_light = map_seed(fertilizer_to_water, &input.water_to_light)?;
    let light_to_temperature = map_seed(water_to_light, &input.light_to_temperature)?;
    let temperature_to_humidity = map_seed(light_to_temperature, &input.temperature_to_humidity)?;
    let humidity_to_location = map_seed(temperature_to_humidity, &input.humidity_to_location)?;
    Ok(humidity_to_location)
}

fn main() -> anyhow::Result<()> {
    let inputs = parse_input(INPUT_DAY5)?;
    log::info!("Done parsing...");
    let locs: Vec<usize> = inputs
        .seeds
        .par_chunks_exact(2)
        .map(|v| {
            let locs: Vec<usize> = (v[0]..v[0] + v[1])
                .map(|seed| get_seed_loc(&inputs, seed).unwrap())
                .collect();
            locs.into_iter().min().unwrap()
        })
        .collect();
    let min_loc: usize = locs
        .into_iter()
        .fold(usize::MAX, |loc_min, loc| loc.min(loc_min));
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
    fn day5part2_solution_test() {
        init_logger();
        let inputs = parse_input(INPUT_DAY5).unwrap();
        let locs: Vec<usize> = inputs
            .seeds
            .par_chunks_exact(2)
            .map(|v| {
                let locs: Vec<usize> = (v[0]..v[0] + v[1])
                    .map(|seed| get_seed_loc(&inputs, seed).unwrap())
                    .collect();
                locs.into_iter().min().unwrap()
            })
            .collect();
        let min_loc: usize = locs
            .into_iter()
            .fold(usize::MAX, |loc_min, loc| loc.min(loc_min));
        assert_eq!(min_loc, 6472060);
    }

    #[test]
    fn day5part2_test() {
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
        let locs: Vec<usize> = inputs
            .seeds
            .par_chunks_exact(2)
            .map(|v| {
                let locs: Vec<usize> = (v[0]..v[0] + v[1])
                    .map(|seed| get_seed_loc(&inputs, seed).unwrap())
                    .collect();
                locs.into_iter().min().unwrap()
            })
            .collect();
        let min_loc: usize = locs
            .into_iter()
            .fold(usize::MAX, |loc_min, loc| loc.min(loc_min));
        assert_eq!(min_loc, 46);
    }
}
