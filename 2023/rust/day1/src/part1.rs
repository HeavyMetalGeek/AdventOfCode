use aoc_lib::INPUT_DAY1;

fn get_first_last_nums(input: &str) -> anyhow::Result<Vec<[u32; 2]>> {
    let lines: Vec<&str> = input.split('\n').collect();
    let nums: Vec<[u32; 2]> = lines
        .into_iter()
        .map(|line| {
            let nums: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            log::debug!("Line: {}, Found: {:?}", line, nums);
            let first = nums.first().unwrap_or(&0_u32).to_owned();
            let last = nums.last().unwrap_or(&0_u32).to_owned();
            [first, last]
        })
        .collect();
     Ok(nums)
}

fn get_sum(nums: Vec<[u32; 2]>) -> u32 {
    nums.into_iter().map(|[f, l]| f * 10 + l).sum()
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let input = INPUT_DAY1;
    let nums = get_first_last_nums(input)?;
    let sum = get_sum(nums);
    println!("The answer is: {}", sum);
    Ok(())
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
