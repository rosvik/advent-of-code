use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            let line = line.unwrap();

            // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            // only 12 red cubes, 13 green cubes, and 14 blue cubes
            let game_num = get_game_num(&line);

            let picks: Vec<&str> = line.split(':').nth(1).unwrap().split(';').collect();

            let mut is_valid = true;

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

                println!(
                    "{}, Red: {}, Green: {}, Blue: {}",
                    game_num, red, green, blue
                );

                if red > 12 || green > 13 || blue > 14 {
                    println!("Invalid game: {}", game_num);
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                sum += game_num;
            }
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
