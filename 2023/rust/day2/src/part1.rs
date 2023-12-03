use aoc_lib::INPUT_DAY2;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<[u32; 3]>,
}

impl Game {
    pub fn new(id: u32, sets: Vec<String>) -> Self {
        let sets: Vec<[u32; 3]> = sets
            .into_iter()
            .map(|set| {
                let mut rgb = [0_u32, 0_u32, 0_u32];
                let parts = set.split(',').collect::<Vec<&str>>();
                for part in parts.into_iter() {
                    if let Some(red) = part.strip_suffix(" red") {
                        rgb[0] = red.trim().parse::<u32>().expect("Failed to parse value");
                        continue;
                    } else if let Some(green) = part.strip_suffix(" green") {
                        rgb[1] = green.trim().parse::<u32>().expect("Failed to parse value");
                        continue;
                    } else if let Some(blue) = part.strip_suffix(" blue") {
                        rgb[2] = blue.trim().parse::<u32>().expect("Failed to parse value");
                        continue;
                    } else {
                        panic!("Something didn't match: {:?}", part);
                    }
                }
                rgb
            })
            .collect();
        return Self { id, sets };
    }

    pub fn sum_sets(&self) -> [u32; 3] {
        return self.sets.iter().fold([0_u32, 0_u32, 0_u32], |sum, set| {
            [sum[0] + set[0], sum[1] + set[1], sum[2] + set[2]]
        });
    }
}


fn main() -> anyhow::Result<()> {
    env_logger::init();
    let input = INPUT_DAY2;
    let games = process_input(input);
    let result = get_result_from_games(games, [12, 13, 14]);
    println!("Result: {}", result);
    return Ok(());
}

fn process_input(input: &str) -> Vec<Game> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let info = line.split(&[':', ';']).collect::<Vec<&str>>();
            let id: u32 = (info[0][5..]).parse::<u32>().unwrap();
            let sets: Vec<String> = info[1..].into_iter().map(|set| set.to_string()).collect();
            Game::new(id, sets)
        })
        .collect();
    log::trace!("{:?}", games);
    return games;
}

// This (incorrectly) assumes all sets cannot contain cubes from other sets
fn get_result_from_games_alt(games: Vec<Game>, rgb_totals: [u32; 3]) -> u32 {
    let result: u32 = games
        .into_iter()
        .inspect(|game| log::debug!("Game {}: {:?}", game.id, game.sum_sets()))
        .filter(|game| {
            let sum = game.sum_sets();
            sum[0] <= rgb_totals[0] && sum[1] <= rgb_totals[1] && sum[2] <= rgb_totals[2]
        })
        .map(|game| game.id)
        .sum();
    return result;
}

fn get_result_from_games(games: Vec<Game>, rgb_totals: [u32; 3]) -> u32 {
    let result: u32 = games
        .into_iter()
        .inspect(|game| log::debug!("Game {}: {:?}", game.id, game.sum_sets()))
        .filter(|game| {
            !game.sets.iter().any(|set| {
                set[0] > rgb_totals[0] || set[1] > rgb_totals[1] || set[2] > rgb_totals[2]
            })
        })
        .map(|game| game.id)
        .sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_test() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        let games = process_input(input);
        let result = get_result_from_games(games, [12, 13, 14]);
        assert_eq!(result, 8);
    }
}
