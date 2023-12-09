use std::fs;
use std::thread;

fn main() {
    part1();
    part2();
}

fn build_input(file : String) -> Vec<Vec<i64>> {
    let contents = fs::read_to_string(file).expect("Success");
    let lines : Vec<&str> = contents.lines().collect();
    let mut numbers : Vec<Vec<i64>> = vec![];

    for line in lines {
        let mut temp : Vec<i64> = vec![];
        for num in line.split(" ") {
            temp.push(num.parse::<i64>().unwrap());
        }
        numbers.push(temp);
    }
    numbers
}

fn part1() {
    let numbers = build_input("inputs/input_9.txt".to_string());
    thread::scope(|thr_scope| {
        let mut my_children = vec![];

        for oasis_line in numbers {
            my_children.push(thr_scope.spawn(|| {
                let mut ladder : Vec<i64> = vec![];
                let mut current = oasis_line;
                ladder.push(current.last().unwrap().clone());
                
                // Build filter ladder down
                while current.iter().fold(0, |acc, x| acc + x) != 0 {
                    let mut t : Vec<i64> = vec![];
                    for i in 0..current.len()-1 {
                        t.push(current[i+1] - current[i]);
                    }
                    ladder.push(t.last().unwrap().clone());
                    current = t
                }
                let mut num = 0;
                for i in ladder[0..ladder.len()-1].iter().rev() {
                    let left = i; 
                    num = left + num;
                }
                num
            }));
        }
        let data = my_children.into_iter().map(|thr| thr.join().unwrap()).fold(0, |acc, x| acc + x);
        println!("Part 1: {}", data); 
    });
}

fn part2() {
    let numbers = build_input("inputs/input_9.txt".to_string());
    thread::scope(|thr_scope| {
        let mut my_children = vec![];

        for oasis_line in numbers {
            my_children.push(thr_scope.spawn(|| {
                let mut ladder : Vec<i64> = vec![];
                let mut current = oasis_line;
                ladder.push(current.first().unwrap().clone());
                
                while current.iter().fold(0, |acc, x| acc + x) != 0 {
                    let mut t : Vec<i64> = vec![];
                    for i in 0..current.len()-1 {
                        t.push(current[i+1] - current[i]);
                    }
                    ladder.push(t.first().unwrap().clone());
                    current = t
                }
                let mut num = 0;
                for i in ladder[0..ladder.len()-1].iter().rev() {
                    num = i - num;
                }
                num
            }));
        }
        let data = my_children.into_iter().map(|thr| thr.join().unwrap()).fold(0, |acc, x| acc + x);
        println!("Part 1: {}", data); 
    });
}
