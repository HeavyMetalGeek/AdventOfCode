fn main() -> anyhow::Result<()> {}

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
