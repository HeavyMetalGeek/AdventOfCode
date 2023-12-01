use aoc_lib::{NumWord, INPUT_DAY1};

fn get_line_values(input: &str) -> [u32; 2] {
    let input_chars: Vec<char> = input.chars().collect();
    let end = input_chars.len() - 1;
    let mut nums = Vec::new();
    let mut i = 0;
    loop {
        if i > end {
            let first = nums.first().unwrap_or(&0_u32).to_owned();
            let last = nums.last().unwrap_or(&0_u32).to_owned();
            log::trace!("moving on...");
            return [first, last];
        }
        let c = input_chars[i];
        if let Some(d) = c.to_digit(10) {
            nums.push(d);
            log::trace!("Found {} at {}", d, i);
        }
        if let Some(numword) = NumWord::from_chars(&input_chars[i..]) {
            nums.push(numword.value);
            log::trace!("Found {} at {}", numword.value, i);
        }
        i += 1;
    }
}

fn get_sum(lines: Vec<&str>) -> u32 {
    let mut sum = 0;
    let sum: u32 = lines
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| line.len() != 0)
        .map(|line| {
            let values = get_line_values(line);
            sum += values[0] * 10 + values[1];
            log::debug!("Line: {}, Result: {:?}, Sum: {:?}", line, values, sum);
            values[0] * 10 + values[1]
        })
        .fold(0, |sum, n| sum + n);
    return sum;
}

fn input_to_lines(input: &str) -> Vec<&str> {
    return input
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| line.len() != 0)
        .collect();
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let input = INPUT_DAY1;

    let lines = input_to_lines(input);
    log::debug!("Lines: {:#?}", lines);

    let sum = get_sum(lines);
    println!("The answer is: {}", sum);
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let lines = input_to_lines(input);
        let sum = get_sum(lines);
        assert_eq!(sum, 281);
    }
}
