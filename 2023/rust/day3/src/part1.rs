use aoc_lib::INPUT_DAY3;
use glam;

#[derive(Debug)]
struct NumBlock {
    value: u32,
    start: glam::UVec2,
    end: glam::UVec2,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let input = INPUT_DAY3;
    let symbol_xys = get_symbol_xys(input);
    let nums = get_numblocks(input);
    let part_numbers = get_part_numbers(nums, symbol_xys);
    println!("Result: {}", part_numbers.iter().sum::<u32>());
    return Ok(());
}

fn get_symbol_xys(input: &str) -> Vec<glam::UVec2> {
    let symbol_positions: Vec<glam::UVec2> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| match ch {
                    '.' => None,
                    ch if ch.is_digit(10) => None,
                    _ => Some(glam::uvec2(x as u32, y as u32)),
                })
                .collect::<Vec<_>>()
        })
        .inspect(|sym| {
            log::debug!("Found symbols: {:?}", sym);
        })
        .collect();
    return symbol_positions;
}

fn get_numblocks(input: &str) -> Vec<NumBlock> {
    let blocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut numblocks = Vec::new();
            let chars: Vec<char> = line.chars().collect();
            let mut x = 0;
            while x < chars.len() {
                if let Some(digit) = chars[x].to_digit(10) {
                    let start = glam::uvec2(x as u32, y as u32);
                    let mut digits = vec![digit];
                    while let Some(more_digit) = chars[x + 1].to_digit(10) {
                        digits.push(more_digit);
                        x += 1;
                        if x == chars.len() - 1 {
                            break;
                        }
                    }
                    let end = glam::uvec2(x as u32, y as u32);
                    let value = digits.into_iter().fold(0, |num, v| num * 10 + v);
                    numblocks.push(NumBlock { start, end, value });
                }
                x += 1;
            }
            numblocks
        })
        .inspect(|blk| {
            log::debug!("Found numblocks: {:?}", blk);
        })
        .collect();
    return blocks;
}


fn get_part_numbers(nums: Vec<NumBlock>, symbol_xys: Vec<glam::UVec2>) -> Vec<u32> {
    let mut part_nums = Vec::new();
    for num in nums {
        // prevent underflow
        let xmin = match num.start.x.checked_sub(1) {
            Some(v) => v,
            None => 0,
        };
        let ymin = match num.start.y.checked_sub(1) {
            Some(v) => v,
            None => 0,
        };
        // prevent overflow
        let xmax = match num.end.x.checked_add(1) {
            Some(v) => v,
            None => u32::MAX,
        };
        let ymax = match num.end.y.checked_add(1) {
            Some(v) => v,
            None => u32::MAX,
        };
        for sym in symbol_xys.iter() {
            let sym_adj_x = sym.x >= xmin && sym.x <= xmax;
            let sym_adj_y = sym.y >= ymin && sym.y <= ymax;
            if sym_adj_x && sym_adj_y {
                part_nums.push(num.value);
                break;
            }
        }
    }
    return part_nums;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day3_part1_test() {
        init_logger();
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        let symbols = get_symbol_xys(input);
        let nums = get_numblocks(input);
        let part_nums = get_part_numbers(nums, symbols);
        log::debug!("Part numbers: {:?}", part_nums);
        assert_eq!(part_nums.iter().sum::<u32>(), 4361);
    }
}
