
fn main() -> anyhow::Result<()> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let nums = get_first_last_nums(input).unwrap();
        let sum = get_sum(nums);
        assert_eq!(sum, 142);
    }
}
