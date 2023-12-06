use std::fs;
use std::collections::HashMap;
use regex::Regex;
use std::thread;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("inputs/input_5.txt").expect("Success");
    let lines : Vec<&str> = contents.lines().collect();
    let mut maps : HashMap::<usize, Vec<Vec<i64>>> = HashMap::new();

    let number_row = Regex::new(r"(\d+)").unwrap();
    let map_name = Regex::new(r"(?<name>.*):").unwrap();
    
    let mut current_name : usize = 0;
    
    for line in &lines[1..] {
        if number_row.is_match(line) { 
            let mut v : Vec<i64> = Vec::new();
            for (_, [num] ) in number_row.captures_iter(line).map(|num| num.extract()) {
                v.push(num.parse::<i64>().unwrap()); 
            }
            let key = current_name;
            let r = maps.get_mut(&key).unwrap();
            r.push(v);
            
        } else if map_name.is_match(line) {
            current_name = current_name + 1;
            maps.insert(current_name, Vec::new());
        }
    }
    
    let seeds_text = lines.first().unwrap();

    let mut results : Vec<i64> = Vec::new();

    for (_, [seed_text]) in number_row.captures_iter(seeds_text).map(|num| num.extract()) {
        let seed = seed_text.parse::<i64>().unwrap();
        let mut m = 1;
        let mut me = seed;
        while m <= current_name {
            let map = maps.get(&m).unwrap();
            for thi in map {
                let e = *thi.get(0).unwrap();
                let s = *thi.get(1).unwrap();
                let i = *thi.get(2).unwrap();
                if me >= s && me < s + i {
                    me = e + (me - s);
                    break;
                }
            }
            m = m + 1;
        }

        results.push(me);
    }

    let res = results.iter().min().unwrap();

    println!("Part 1: {}", res);
}

fn part2() {
    let contents = fs::read_to_string("inputs/input_5.txt").expect("Success");
    let lines : Vec<&str> = contents.lines().collect();
    let mut maps : HashMap::<usize, Vec<Vec<i64>>> = HashMap::new();

    let number_row = Regex::new(r"(\d+)").unwrap();
    let map_name = Regex::new(r"(?<name>.*):").unwrap();
    
    let mut current_name : usize = 0;
    
    for line in &lines[1..] {
        if number_row.is_match(line) { 
            let mut v : Vec<i64> = Vec::new();
            for (_, [num] ) in number_row.captures_iter(line).map(|num| num.extract()) {
                v.push(num.parse::<i64>().unwrap()); 
            }
            let key = current_name;
            let r = maps.get_mut(&key).unwrap();
            r.push(v);
            
        } else if map_name.is_match(line) {
            current_name = current_name + 1;
            maps.insert(current_name, Vec::new());
        }
    }
    let seeds_text = lines.first().unwrap();
    let seed_row_re = Regex::new(r"(\d+ \d+)").unwrap();

    let mut children = vec![];

    for (_, [seed_text]) in seed_row_re.captures_iter(seeds_text).map(|num| num.extract()) {
        let mmaps = maps.clone();
        children.push(thread::spawn(move || {
            let seed_range : Vec<i64> = seed_text.split(" ").collect::<Vec<&str>>().iter_mut().map(|g| g.parse::<i64>().unwrap()).collect();
            let seed_start = seed_range[0];
            let seed_end = seed_range[0] + seed_start;

            let mut results : Vec<i64> = Vec::new();

            for seed in seed_start..seed_end {
                let mut m = 1;
                let mut me = seed;
                while m <= current_name {
                    let map = mmaps.get(&m).unwrap();
                    for thi in map {
                        let e = *thi.get(0).unwrap();
                        let s = *thi.get(1).unwrap();
                        let i = *thi.get(2).unwrap();
                        if me >= s && me < s + i {
                            me = e + (me - s);
                            break;
                        }
                    }
                    m = m + 1;
                }
                results.push(me);
            }

            results.iter().min().unwrap();
        }));
    }

    let _min = children.into_iter().map(|child| {
        child.join().unwrap()
    });
}
