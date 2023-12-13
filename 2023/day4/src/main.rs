use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut sum = 0;
        for line in lines {
            let line = line.unwrap();
            let number_part = line.split(": ").last().unwrap();

            let winning_part = number_part.split(" | ").next().unwrap();
            let your_part = number_part.split(" | ").last().unwrap();

            let winning_part: Vec<u32> = winning_part
                .split(' ')
                .map(|x| x.trim().parse::<u32>().unwrap_or(0))
                .collect();

            let your_part: Vec<u32> = your_part
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap_or(0))
                .collect();

            let mut card_score: u32 = 0;
            winning_part.iter().for_each(|i| {
                your_part.iter().for_each(|j| {
                    if i == j {
                        if card_score == 0 {
                            card_score = 1;
                        } else {
                            card_score *= 2;
                        }
                    }
                });
            });
            println!("{:?} | {:?} -> {}", winning_part, your_part, card_score);

            sum += card_score;
        }
        println!("SUM: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
