use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            let line = line.unwrap();
            println!("{}", line);

            // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            let game_num = get_game_num(&line);

            let picks: Vec<&str> = line.split(':').nth(1).unwrap().split(';').collect();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for pick in picks {
                let cubes: Vec<&str> = pick.split(',').collect();

                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                for cube in cubes {
                    let cube = cube.trim();
                    if cube.ends_with("red") {
                        red += cube.split(' ').next().unwrap().parse::<i32>().unwrap();
                    } else if cube.ends_with("green") {
                        green += cube.split(' ').next().unwrap().parse::<i32>().unwrap();
                    } else if cube.ends_with("blue") {
                        blue += cube.split(' ').next().unwrap().parse::<i32>().unwrap();
                    }
                }

                if red > max_red {
                    max_red = red;
                }
                if green > max_green {
                    max_green = green;
                }
                if blue > max_blue {
                    max_blue = blue;
                }
            }

            println!("Game {}: {} {} {}", game_num, max_red, max_green, max_blue);

            sum += max_red * max_green * max_blue;
        }
        println!("SUM: {}", sum);
    }
}

fn get_game_num(line: &str) -> i32 {
    line.split(':')
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
