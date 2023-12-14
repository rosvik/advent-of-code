use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut _sum = 0;

        let mut prev_line: String;
        let mut current_line = String::from("");
        let mut next_line = String::from("");

        for line in lines {
            let line = line.unwrap();
            prev_line = current_line.clone();
            current_line = next_line.clone();
            next_line = line;

            _sum += calc_line(&current_line, &prev_line, &next_line);
        }

        prev_line = current_line.clone();
        current_line = next_line.clone();
        next_line = String::from("");

        _sum += calc_line(&current_line, &prev_line, &next_line);

        println!("SUM: {}", _sum);
    }
}

fn calc_line(line: &String, prev: &String, next: &String) -> u32 {
    let mut sum = 0;

    let symbols: Vec<char> = [
        '*', '$', '+', '/', '-', '%', '^', '&', '(', ')', '=', '!', '?', '@', '#', '~', '`', '{',
    ]
    .to_vec();

    let nums = match get_line_numbers(line) {
        Some(l) => l,
        None => return 0,
    };

    for (num, start, end) in nums {
        // check if num is in prev or next line
        let start_with_offset: usize = (start - 1).clamp(0, line.len() as i32) as usize;
        let end_with_offset: usize = (end + 2).clamp(0, line.len() as i32) as usize;

        let prev_range: &str = match prev.is_empty() {
            true => "" as &str,
            false => &prev[start_with_offset..end_with_offset],
        };
        let current_range: &str = &line[start_with_offset..end_with_offset];
        let next_range: &str = match next.is_empty() {
            true => "" as &str,
            false => &next[start_with_offset..end_with_offset],
        };

        let chars_next_to = format!("{}{}{}", prev_range, current_range, next_range);

        if chars_next_to.chars().any(|c| symbols.contains(&c)) {
            sum += num as u32;
        }
    }

    sum
}

fn get_line_numbers(line: &str) -> Option<Vec<(i32, i32, i32)>> {
    let mut nums = Vec::<(i32, i32, i32)>::new();
    let mut num_str = String::from("");
    let mut start_index = -1;
    let mut end_index = 0;
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if start_index == -1 {
                start_index = i as i32;
            }
            end_index = i as i32;
            num_str += &c.to_string();
        } else if !num_str.is_empty() {
            let num = num_str.parse::<i32>().unwrap();
            nums.push((num, start_index, end_index));

            // reset
            num_str = String::from("");
            start_index = -1;
            end_index = 0;
        }
    }

    // last number
    if !num_str.is_empty() {
        let num = num_str.parse::<i32>().unwrap();
        nums.push((num, start_index, end_index));
    }

    Some(nums)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
