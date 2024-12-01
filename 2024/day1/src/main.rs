use std::collections::HashMap;
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

    // Count occurrences in right list
    let mut right_counts: HashMap<i64, i64> = HashMap::new();
    for &num in &right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score
    let similarity_score: i64 = left_list
        .iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {}", similarity_score);

    Ok(())
}
