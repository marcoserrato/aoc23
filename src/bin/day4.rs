use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input_4.txt").expect("Success");
    let total = contents.lines().fold(0, |acc : i64, line| {
        let sp : Vec<&str> = line.split("|").collect();
        let card = sp[0];
        let winning = sp[1];
        let cards_and_numbers : Vec<&str> = card.split(":").collect();
        let card_numbers : Vec<&str> = cards_and_numbers[1].split(" ").collect();
        let winning_numbers : Vec<&str> = winning.split(" ").collect();
        let mut matched = 0;
        for num in card_numbers {
            match num {
                "" => {},
                _ => {
                    for win_num in &winning_numbers {
                        if num == *win_num {
                            matched = matched + 1;
                        }
                    }
                }
            }
        }
        if matched > 0 {
            let base : i64 = 2;
            acc + base.pow(matched - 1)
        } else {
            acc
        }
    });
    println!("Part 1: {}", total);
}

fn part2() {
    let contents = fs::read_to_string("input_4.txt").expect("Success");
    let input : Vec<&str> = contents.lines().collect();
    let mut cards : Vec<i32> = vec![1; input.len()];
    let mut total = 0;

    for (ind, line) in input.iter().enumerate() {
        let sp : Vec<&str> = line.split("|").collect();
        let card = sp[0];
        let winning = sp[1];
        let cards_and_numbers : Vec<&str> = card.split(":").collect();
        let card_numbers : Vec<&str> = cards_and_numbers[1].split(" ").collect();
        let winning_numbers : Vec<&str> = winning.split(" ").collect();
        let mut matched = 0;
        for num in card_numbers {
            match num {
                "" => {},
                _ => {
                    for win_num in &winning_numbers {
                        if num == *win_num {
                            matched = matched + 1;
                        }
                    }
                }
            }
        }

        let current_multiplier = cards[ind];
        for i in ind + 1..ind + 1 + matched {
            cards[i] = cards[i] + (1 * current_multiplier);
        };

        total += cards[ind];
    }

    println!("Part 2: {}", total);
}
