use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut x = 1;
    let mut cycle = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let linstring = line.unwrap();
            let mut args = linstring.split(" ").collect::<Vec<&str>>();

            if let Some(comd) = args.first() {
                cycle += 1;

                check_cycle(cycle, x);

                if comd == &"noop" {
                    continue;
                }

                if comd == &"addx" {
                    cycle += 1;
                    check_cycle(cycle, x);

                    if let Some(n) = args.pop() {
                        if let Ok(num) = n.parse::<i32>() {
                            x += num;
                        }
                    }

                    continue;
                }
            }
        }
    }
    println!("")
}

fn check_cycle(cycle: i32, x: i32) {
    let cycles = [20, 60, 100, 140, 180, 220];

    let signal_strength = cycle * x;

    if cycles.contains(&cycle) {
        print!("{} + ", signal_strength)
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
