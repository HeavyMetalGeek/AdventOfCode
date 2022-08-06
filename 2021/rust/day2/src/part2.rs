use std::fs;
use std::path::Path;

fn main() {
    // Get depths from file
    //let fpath = Path::new("./input/test.json");
    let fpath = Path::new("./input/moves.json");
    let move_str = fs::read_to_string(fpath).expect("File read failed.");
    let moves: Vec<&str> = serde_json::from_str(&move_str).expect("Failed to deserialize.");

    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;
    for mov in moves {
        let parts: Vec<String> = mov.split(' ').map(|s| s.trim().to_string()).collect();
        if parts[0] == "forward" {
            let x = parts[1].parse::<i32>().unwrap();
            h += x;
            v += aim * x;
        } else if parts[0] == "up" {
            aim -= parts[1].parse::<i32>().unwrap();
        } else if parts[0] == "down" {
            aim += parts[1].parse::<i32>().unwrap();
        }
    }
    let pos = h * v;
    println!("Postition: h:{h} * v:{v} = {pos}");
}
