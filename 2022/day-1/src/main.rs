use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1(1);
    part2(1);
}

fn part2(verbosity: u32) {
    if let Ok(lines) = read_lines("./input") {
        let mut current_cals: i32 = 0;
        let mut all_elves = vec![];

        for line in lines {
            if let Ok(n) = line {
                if !n.is_empty() {
                    let num = n.parse::<i32>().unwrap();
                    current_cals += num;
                    if verbosity > 1 {
                        print!("{} + ", num);
                    }
                } else {
                    if verbosity > 1 {
                        println!("= {}", current_cals);
                    }
                    all_elves.push(current_cals);
                    current_cals = 0;
                }
            }
        }
        if verbosity > 1 {
            println!("= {}", current_cals);
        }

        all_elves.sort();
        all_elves.reverse();

        let sum: i32 = all_elves.iter().take(3).sum();

        if verbosity > 1 {
            println!("Sorted, and reversed: {:?}", all_elves);
        }

        println!("Part 2: Total of largest three was {}", sum);
    }
}

fn part1(verbosity: u32) {
    let mut line_number = 0;

    if let Ok(lines) = read_lines("./input") {
        let mut elf_carrying: i32 = 0;
        let mut elf_most_num: Option<i32> = None;
        let mut elf_most_weight: i32 = 0;

        for line in lines {
            line_number += 1;
            if let Ok(n) = line {
                if !n.is_empty() {
                    let num = n.parse::<i32>().unwrap();
                    elf_carrying += num;
                    if verbosity > 1 {
                        print!("{} + ", num);
                    }
                } else {
                    if verbosity > 1 {
                        println!("= {}", elf_carrying);
                    }
                    if elf_carrying > elf_most_weight {
                        elf_most_weight = elf_carrying;
                        elf_most_num = Some(line_number);
                    }
                    elf_carrying = 0;
                }
            }
        }
        if verbosity > 1 {
            println!("= {}", elf_carrying);
        }

        match elf_most_num {
            Some(x) => println!("Part 1: Most calories was {elf_most_weight} (line {x})\n"),
            None => println!("Part 1: Not found\n"),
        }
    }
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
