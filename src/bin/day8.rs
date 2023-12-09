use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("inputs/input_8.txt").expect("Success");
    let mut map : HashMap<String, (String, String)> = HashMap::new();
    let map_line_re = Regex::new(r"(?<source>\w{3}).*(?<left>\w{3}).*(?<right>\w{3})").unwrap();
    
    let lines : Vec<&str> = contents.lines().collect();

    let directions = lines.first().unwrap();
    let direction_chars : Vec<char> = directions.chars().collect();

    for line in contents.lines() {
        let line_captures = map_line_re.captures(line);

        match line_captures {
            Some(cap) => {
                map.insert(cap["source"].to_string(), (cap["left"].to_string(), cap["right"].to_string()));
            },
            _ => { }
        }
    }

    let mut current_key = "AAA";

    let mut count = 0;

    while current_key != "ZZZ" {
        for direction in direction_chars.iter() {
            let current_position = map.get(current_key).unwrap();
            match direction {
                'R' => {
                    let next =  &current_position.1;
                    count += 1;
                    current_key = next;
                },
                'L' => {
                    let next = &current_position.0;
                    count += 1;
                    current_key = next;
                }
                _ => { println!("Not a valid direction!") }
            }
            if current_key == "ZZZ" {
                break
            }
        }
    }

    println!("Part 1: {}", count);
}

fn part2() {
}
