use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    increases();
    sliding_window();
}

fn sliding_window() {
    let mut measurements: LinkedList<u32> = LinkedList::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(n) = line {
                let current: u32 = n.parse().unwrap();

                measurements.push_back(current);
            }
        }
    }

    println!("{:?}", measurements.pop_back().clone())
}

fn increases() {
    let mut count: i32 = 0;
    let mut prevint = 999999;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(n) = line {
                let measure: i32 = n.parse().unwrap();
                if measure > prevint {
                    count = count + 1;
                }
                prevint = measure;
            }
        }
    }
    println!("{}", count)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
