use std::fs;
use std::path::Path;

fn main() {
    // Get depths from file
    //let fpath = Path::new("./input/test.json");
    let fpath = Path::new("./input/depths.json");
    let depth_str = fs::read_to_string(fpath).unwrap();
    let depths: Vec<i32> = serde_json::from_str(&depth_str).unwrap();

    let cnt = depths
        .windows(4)
        .map(|c| c[1..4].iter().sum::<i32>() - c[0..3].iter().sum::<i32>())
        .filter(|&d| d > 0)
        .fold(0, |acc, _| acc + 1);
    println!("Increases: {cnt}");
}
