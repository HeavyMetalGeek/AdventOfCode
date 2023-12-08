use aoc_lib::INPUT_DAY4;

#[derive(Debug)]
pub struct Card {
    number: u32,
    win_values: Vec<u32>,
    values: Vec<u32>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let start = std::time::Instant::now();
    let input = INPUT_DAY4;
    let cards = parse_cards(input);
    log::debug!("Cards: {}", cards.len());
    let winner_count = count_winners(&cards, 0, cards.len());
    let run_time = std::time::Instant::now().duration_since(start).as_millis();
    //let total_points = cards.iter().map(card_points).sum::<u32>();
    println!("Result: {}", winner_count);
    println!("Run time: {} ms", run_time);
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

fn card_winners(card: &Card) -> usize {
    use std::collections::HashSet;
    let win_values: HashSet<u32> = HashSet::from_iter(card.win_values.iter().cloned());
    let values: HashSet<u32> = HashSet::from_iter(card.values.iter().cloned());
    win_values.intersection(&values).collect::<Vec<_>>().len()
}

fn count_winners(cards: &[Card], index: usize, count: usize) -> usize {
    let mut card_count = count;
    for (i, card) in cards[index..index + count].iter().enumerate() {
        let winners = card_winners(card);
        log::debug!("Card {} has {} winners", card.number, winners);
        if winners == 0 {
            continue;
        }
        card_count += count_winners(cards, index + i + 1, winners);
    }
    card_count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day4_part2_test() {
        init_logger();
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let cards = parse_cards(input);
        let winner_count = count_winners(&cards, 0, cards.len());
        assert_eq!(winner_count, 30);
    }
}

