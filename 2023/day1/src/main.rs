use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut sum = 0;
        for line in lines {
            let line = line.unwrap();

            let number_chars = get_number_chars(&line);
            let first = number_chars.chars().next().unwrap();
            let last = number_chars.chars().last().unwrap();

            let first_last_num_string = format!("{}{}", first, last);
            let first_last_num: u32 = first_last_num_string.parse().unwrap();

            sum += first_last_num;
        }
        println!("SUM: {}", sum);
    }
}

fn get_number_chars(line: &str) -> String {
    let mut numbers: String = String::new();
    let num_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, ch) in line.chars().enumerate() {
        if ch.is_numeric() {
            numbers.push(ch);
        } else if ch.is_alphabetic() {
            let rest_of_line = line[i..].to_string();
            for (i, num_word) in num_words.iter().enumerate() {
                if rest_of_line.starts_with(num_word) {
                    numbers.push_str(&i.to_string());
                    break;
                }
            }
        }
    }

    println!("line: {}, numbers: {}", line, numbers);

    numbers
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
