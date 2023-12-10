use aoc_lib::INPUT_DAY4;

#[derive(Debug)]
pub struct Card {
    number: u32,
    win_values: Vec<u32>,
    values: Vec<u32>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let input = INPUT_DAY4;
    let cards = parse_cards(input);
    let total_points = cards.iter().map(card_points).sum::<u32>();
    println!("Result: {}", total_points);
    Ok(())
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|card| {
            let parts: Vec<&str> = card.split(&[':', '|']).collect();
            log::debug!("parts: {:?}", parts);
            assert!(parts.len() == 3);
            let number: u32 = parts[0]
                .split(' ')
                .filter_map(|v| v.trim().parse::<u32>().ok())
                .collect::<Vec<u32>>()[0];
            let win_values = parts[1]
                .split(' ')
                .filter_map(|v| v.trim().parse::<u32>().ok())
                .collect();
            let values = parts[2]
                .split(' ')
                .filter_map(|v| v.trim().parse::<u32>().ok())
                .collect();
            Card {
                number,
                win_values,
                values,
            }
        })
        .inspect(|card| log::debug!("{:?}", card))
        .collect()
}

fn card_points(card: &Card) -> u32 {
    use std::collections::HashSet;
    let win_values: HashSet<u32> = HashSet::from_iter(card.win_values.iter().cloned());
    let values: HashSet<u32> = HashSet::from_iter(card.values.iter().cloned());
    win_values.intersection(&values).fold(0_u32, |total, v| if total == 0 {
        total + 1
    } else {
        total * 2
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day4part1_test() {
        init_logger();
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let cards = parse_cards(input);
        let total_points = cards.iter().map(card_points).sum::<u32>();
        assert_eq!(total_points, 13);
    }
}
