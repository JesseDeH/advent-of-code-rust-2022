use std::env::{self, Args};

mod solutions;

fn main() {
    let args = parse_args(env::args());

    match args {
        None => println!("Provide the day and part of the challenge."),
        Some((day, part)) => solve(day, part),
    };
}

fn parse_args(raw_args: Args) -> Option<(i32, i32)> {
    let args: Vec<String> = raw_args.collect();
    match args.len() {
        3 => {
            return Some((
                args[1].parse::<i32>().unwrap_or(1),
                args[2].parse::<i32>().unwrap_or(1),
            ));
        }
        _ => return None,
    }
}

fn solve(day: i32, part: i32) {
    println!("Day {}, Part {}", day, part);

    match (day, part) {
        (1, 1) => println!(
            "{}",
            solutions::day01::solve_part1(include_str!("solutions/input/day01.txt"))
        ),
        (1, 2) => println!(
            "{}",
            solutions::day01::solve_part2(include_str!("solutions/input/day01.txt"))
        ),
        (3, 1) => println!(
            "{}",
            solutions::day03::solve_part1(include_str!("solutions/input/day03.txt"))
        ),
        (3, 2) => println!(
            "{}",
            solutions::day03::solve_part2(include_str!("solutions/input/day03.txt"))
        ),
        (4, 1) => println!(
            "{}",
            solutions::day04::solve_part1(include_str!("solutions/input/day04.txt"))
        ),
        (4, 2) => println!(
            "{}",
            solutions::day04::solve_part2(include_str!("solutions/input/day04.txt"))
        ),
        (5, 1) => println!(
            "{}",
            solutions::day05::solve_part1(include_str!("solutions/input/day05.txt"))
        ),
        (5, 2) => println!(
            "{}",
            solutions::day05::solve_part2(include_str!("solutions/input/day05.txt"))
        ),
        _ => println!("not found"),
    }
}
