fn main() {
    part1();
    part2();
}

fn part1() {
    // Why parse the input when it's so small
    let map = [(7,9), (15,40), (30,200)];
    let mut ranges : Vec<i32> = vec!();
    for (time, dist) in map {
        let _variable : f64 = ((time as i64).pow(2) - (4 * dist)) as f64;
        let variable : f64 = _variable.sqrt();

        let mut point1 = (-time as f64 - variable) / -2 as f64;
        let mut point2 = (-time as f64 + variable) / -2 as f64;

        if point1 > point2 {
            point1 = (point1 - 1 as f64).ceil();
            point2 = (point2 + 1 as f64).floor();

        } else {
            point1 = (point1 + 1 as f64).ceil();
            point2 = (point2 - 1 as f64).floor();
        }

        let range = (point2 - point1).abs() + 1 as f64;
        ranges.push(range as i32);
    }

    println!("Part 1: {}", ranges.iter().fold(1, |acc, x| { acc * x }));
}

fn part2() {
    // Why parse the input when it's so small
    let map = [(71530,940200)];
    let mut ranges : Vec<i32> = vec!();
    for (time, dist) in map {
        let _variable : f64 = ((time as i64).pow(2) - (4 * dist)) as f64;
        let variable : f64 = _variable.sqrt();

        let mut point1 = (-time as f64 - variable) / -2 as f64;
        let mut point2 = (-time as f64 + variable) / -2 as f64;

        if point1 > point2 {
            point1 = (point1 - 1 as f64).ceil();
            point2 = (point2 + 1 as f64).floor();

        } else {
            point1 = (point1 + 1 as f64).ceil();
            point2 = (point2 - 1 as f64).floor();
        }

        let range = (point2 - point1).abs() + 1 as f64;
        ranges.push(range as i32);
    }

    println!("Part 2: {}", ranges.iter().fold(1, |acc, x| { acc * x }));
}
