use std::fs;

fn main() { 
    assert_eq!(process_forward("one1two2three3four4five5six6seven8eight8nine9".to_string()), Some(1));
    assert_eq!(process_backwards("twone".to_string()), Some(1));
    part1();
    part2();
}

fn part1() {
    let mut count: u32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Successfully read file");

    for line in contents.lines() {
        let mut chars = line.chars();
        
        let mut first : u32 = 0;
        while let Some(i) = chars.next() {
            if i.is_digit(10) {
                first = i.to_digit(10).unwrap();
                break;
            }
            else {
            }
        }

        let mut last : u32 = first;
        while let Some(i) = chars.next_back() {
            if i.is_digit(10) {
                last = i.to_digit(10).unwrap();
                break;
            }
            else {
            }
        }

        let chars : Vec<String> = vec!(first.to_string(), last.to_string());
        let sum : String = chars.into_iter().collect();
        let num = sum.parse::<u32>().unwrap();

        count += num
    }   
    println!("Part 1: Total: {count}")
}

fn part2() {
    let mut count: u32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Successfully read file");
    for line in contents.lines() {
        let first = process_forward(line.to_string()).unwrap().to_string();
        let last = process_backwards(line.to_string()).unwrap().to_string();
        let together : String = vec!(first, last).into_iter().collect();
        let num = together.parse::<u32>().unwrap();
        count += num;
    }
    println!("Part 2: Total: {count}")
    
}


// turn ["onetwothree"] into [1,2,3]
fn process_forward(line : String) -> Option<u32> {
    let mut i  = 0;

    while i < line.len() {
        let possible_number = to_number(&line[i..line.len()]);
        match possible_number {
            Some(number) => {
                return Some(number);
            }
            None => {
                let current_char = line.get(i..i+1).unwrap();
                let current_digit = current_char.parse::<u32>();
                match current_digit {
                    Ok(digit) => { 
                        return Some(digit)
                    },
                    Err(_) => {
                        i = i + 1
                    }
                }
            }
        }
    }
    return None;
}

fn process_backwards(line : String) -> Option<u32> {
    let mut i  = i32::try_from(line.len() - 1).unwrap();

    while i >= 0 {
        let possible_number = to_number(&line[usize::try_from(i).unwrap()..line.len()]);
        match possible_number {
            Some(number) => {
                return Some(number);
            }
            None => {
                let current_char = line.get(usize::try_from(i).unwrap()..usize::try_from(i).unwrap()+1).unwrap();
                let current_digit = current_char.parse::<u32>();
                match current_digit {
                    Ok(digit) => { 
                        return Some(digit);
                    },
                    Err(_) => {
                        i = i - 1
                    }
                }
            }
        }
    }
    return None;
}

fn to_number(line: &str) -> Option<u32> {
    if line.starts_with("one") {
        Some(1)
    }
    else if line.starts_with("two") {
        Some(2)
    }
    else if line.starts_with("three") {
        Some(3)
    }
    else if line.starts_with("four") {
        Some(4)
    }
    else if line.starts_with("five") {
        Some(5)
    }
    else if line.starts_with("six") {
        Some(6)
    }
    else if line.starts_with("seven") {
        Some(7)
    }
    else if line.starts_with("eight") {
        Some(8)
    }
    else if line.starts_with("nine") {
        Some(9)
    }
    else {
        None
    }
}
