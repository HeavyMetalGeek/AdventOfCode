use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Get depths from file
    let mut fpath = Path::new("./input/test.json");
    if args.len() > 1 && args[1] == "test" {
        fpath = Path::new("./input/test.json");
    }
    let inputs_str = fs::read_to_string(fpath).expect("File read failed.");
    let inputs: Vec<&str> = serde_json::from_str(&inputs_str).expect("Failed to deserialize.");

    let split_inputs: Vec<Vec<char>> = inputs.iter().map(|i| i.chars().collect()).collect();
    let (gamma, epsilon) = get_output(&split_inputs);
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    let result = gamma * epsilon;
    println!("Result: {}", result);
}

// Number of bits is unknown
fn get_output(input: &[Vec<char>]) -> (u32, u32) {
    let bin_len = input[0].len();
    let mut sums = vec![0; bin_len];
    let mut count = 0;
    for bin in input {
        for (j, col) in sums.iter_mut().enumerate() {
            *col += bin[j].to_digit(10).expect("Failed to convert char to u32");
        }
        count += 1;
    }

    let mut gamma = vec![0; bin_len];
    let mut epsilon = vec![1; bin_len];

    for (i, col) in sums.iter().enumerate() {
        if (*col as f64) / (count as f64) > 0.5 {
            gamma[i] = 1;
            epsilon[i] = 0;
        }
    }

    let gamma_sum = gamma
        .iter()
        .rev()
        .enumerate()
        .map(|i| i.1 * 2_u32.pow(i.0 as u32))
        .sum();
    let epsilon_sum = epsilon
        .iter()
        .rev()
        .enumerate()
        .map(|i| i.1 * 2_u32.pow(i.0 as u32))
        .sum();
    (gamma_sum, epsilon_sum)
}
