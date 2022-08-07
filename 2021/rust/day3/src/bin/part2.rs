use day3::processor::Processor;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Get depths from file
    let mut fpath = Path::new("./input/inputs.json");
    if args.len() > 1 && args[1] == "test" {
        fpath = Path::new("./input/test.json");
    }
    let inputs_str = fs::read_to_string(fpath).expect("File read failed.");
    let inputs: Vec<&str> = serde_json::from_str(&inputs_str).expect("Failed to deserialize.");

    let split_inputs: Vec<Vec<char>> = inputs.iter().map(|i| i.chars().collect()).collect();
    let results = Processor::new(split_inputs);
    println!("Gamma: {}", results.gamma.unwrap());
    println!("Epsilon Rate: {}", results.epsilon.unwrap());
    println!("Power Consumption: {}", results.power_consumption.unwrap());
    println!();
    println!("Oxygen Rating: {}", results.o2_rating.unwrap());
    println!("CO2 Rating: {}", results.co2_rating.unwrap());
    println!(
        "Life Support Rating: {}",
        results.life_support_rating.unwrap()
    );
}
