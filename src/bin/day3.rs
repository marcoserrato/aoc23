use std::fs;

fn main() {
    assert_eq!(find_numbers_and_starting_pos(".242..276"), vec!(("242", 1), ("276", 6)));
    part1();
    part2();
}

fn part2() {
    let contents = fs::read_to_string("input_3.txt")
        .expect("Sucess");

    // this sucks
    let lines : Vec<&str> = contents.lines().into_iter().collect();
    let mut count = 0;
    for (ind, line) in contents.lines().into_iter().enumerate() {
        let gears = find_gears(line);
        for gear in gears {
            let above = if ind > 0 { Some(lines[ind - 1]) } else { None };
            let below = if ind < lines.len() - 1 { Some(lines[ind + 1]) } else { None };
            let adj_nums = find_adjacent_numbers(line, above, below, gear);
            if adj_nums.len() == 2 {
                count += adj_nums.iter().fold(1, |acc, x| acc * x);
            }
        }
    }

    println!("Part 2: {}", count);
}

fn find_adjacent_numbers(current: &str, above: Option<&str>, below: Option<&str>, pos: usize) -> Vec<usize> {
    let mut res : Vec<usize> = Vec::new();
    
    let current_line_numbers = find_numbers_and_starting_pos(current);
    for (number, starting_pos) in current_line_numbers {
        if starting_pos + number.len() == pos {
            res.push(number.parse::<usize>().unwrap());
        } else if starting_pos - 1 == pos {
            res.push(number.parse::<usize>().unwrap());
        }
    }

    match above {
        Some(arr) => {
            let above_numbers = find_numbers_and_starting_pos(arr);
            for (number, starting_pos) in above_numbers {
                let left = if starting_pos == 0 { starting_pos} else { starting_pos - 1 };
                if pos >= left && pos <= starting_pos + number.len() {
                    res.push(number.parse::<usize>().unwrap());
                }
            }
        },
        _ => {}
    }

    match below {
        Some(arr) => {
            let below_numbers = find_numbers_and_starting_pos(arr);
            for (number, starting_pos) in below_numbers {
                let left = if starting_pos == 0 { starting_pos} else { starting_pos - 1 };
                if pos >= left && pos <= starting_pos + number.len() {
                    res.push(number.parse::<usize>().unwrap());
                }
            }
        },
        _ => {}
    }

    return res;
}

fn find_gears(line: &str) -> Vec<usize> {
    let mut res : Vec<usize> = Vec::new();
    for (i, char) in line.chars().into_iter().enumerate() {
        match char {
            '*' => { res.push(i) },
            _ => {}
        }
    }
    return res;
}

fn part1() {
    let contents = fs::read_to_string("input_3.txt")
        .expect("Success");

    // this sucks
    let lines : Vec<&str> = contents.lines().into_iter().collect();
    let mut count = 0;

    for (ind, line) in contents.lines().into_iter().enumerate() {
        let num_and_poses = find_numbers_and_starting_pos(line);
        for (number, starting_pos) in num_and_poses {
            if ind > 0 && check_other_line(lines[ind-1], starting_pos, starting_pos + number.len()) {
                count += number.parse::<i32>().unwrap();
            } else if check_next_to(line, starting_pos, starting_pos + number.len()) {
                count += number.parse::<i32>().unwrap();
            } else if ind < lines.len() - 1 && check_other_line(lines[ind+1], starting_pos, starting_pos + number.len()) {
                count += number.parse::<i32>().unwrap();
            }
        }
    }

    println!("Part 1: {}", count);
}

fn check_other_line(line: &str, start: usize, end: usize) -> bool {
    let sind = if start == 0 { 0 } else { start - 1 };
    let send = if end >= line.len() { end } else { end + 1 };

    return search_for_symbol(&line[sind..send]);
}
fn check_next_to(line: &str, start: usize, end: usize) -> bool {
    if start > 0 && line[start - 1..start] != *"." {
        return true
    } else if end < line.len() && line[end..end + 1] != *"." {
        return true
    } else {
        return false
    }
}

fn search_for_symbol(string: &str) -> bool {
    for char in string.chars() {
        match char.to_string().as_str() {
            "." => {},
            _ => { return true }
        }
    }
    return false;
}


fn find_numbers_and_starting_pos(line: &str) -> Vec<(&str, usize)> {
    let mut result : Vec<(&str, usize)> = Vec::new();
    let bytes = line.chars();

    let mut current : Option<usize> = None;
    for (i, byte) in bytes.into_iter().enumerate() {
        if byte.is_digit(10) {
            match current {
                Some(_) => {},
                None => {
                    current = Some(i);
                }
            }
        } else {
            match current {
                Some(v) => { 
                    result.push((&line[v..i], v));
                    current = None; 
                }
                _ => {}
            }
        }
    }
    match current {
        Some(v) => { result.push((&line[v..line.len()], v)) }
        _ => {}
    }
    return result;
}
