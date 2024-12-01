use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Read numbers from file
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    // Sort both lists to pair smallest with smallest
    left_list.sort();
    right_list.sort();

    // Calculate total distance
    let total_distance: i64 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total distance between lists: {}", total_distance);

    Ok(())
}
