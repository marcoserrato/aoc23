use std::fs;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input_2.txt")
        .expect("Successfully read file");
    let re = Regex::new(r" (\d+) (red|blue|green)").unwrap();
    let re2 = Regex::new(r"Game (\d+)").unwrap();
    let mut count = 0;

    for line in contents.lines() {
        let line_stat: Vec<&str> = line.split(":").collect();
        let games = line_stat[1];
        let game_id = re2.captures(line_stat[0]).unwrap()[1].parse::<i32>().unwrap();
        let throws : Vec<&str> = games.split(";").collect();
        let valid_game = throws.iter().copied().fold(true, |acc : bool, throw : &str| {
            let dice : Vec<&str> = throw.split(',').collect();
            let valid : bool = dice.iter().copied().fold(true, |inner_acc : bool, die : &str| {
                let res = re.captures(die).unwrap();
                let color : &str = &res[2];
                let value : i32 = res[1].parse::<i32>().unwrap();

                match color.to_string().as_str() {
                    "red" => { inner_acc && (value <= 12) },
                    "blue" => { inner_acc && (value <= 14) },
                    "green" => { inner_acc && (value <= 13) },
                    _ => { inner_acc }
                }
            });
            acc && valid
        });
        if valid_game {
            println!("Valid: {}", line);
            count = count + game_id;
        } else {
            println!("Invalid: {}", line)
        }
    }
    println!("Part 1: {}", count);
}

fn part2() {
    let contents = fs::read_to_string("input_2.txt")
        .expect("Successfully read file");
    let re = Regex::new(r" (\d+) (red|blue|green)").unwrap();
    let mut count = 0;

    for line in contents.lines() {
        let line_stat: Vec<&str> = line.split(":").collect();
        let games = line_stat[1];
        let throws : Vec<&str> = games.split(";").collect();

        let mut max_r = 0;
        let mut max_b = 0;
        let mut max_g = 0;
        for throw in throws {
            let dice : Vec<&str> = throw.split(',').collect();
            for die in dice {
                let res = re.captures(die).unwrap();
                let color : &str = &res[2];
                let value : i32 = res[1].parse::<i32>().unwrap();

                match color.to_string().as_str() {
                    "red" => { if value > max_r { max_r = value} },
                    "blue" => { if value > max_b { max_b = value} },
                    "green" => { if value > max_g { max_g = value} },
                    _ => { }
                }
            }
        }
        count = count + (max_r * max_g * max_b)
    }
    println!("Part 1: {}", count);
}
