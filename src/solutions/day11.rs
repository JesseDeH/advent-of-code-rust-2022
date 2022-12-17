use std::{collections::VecDeque, str::Lines};

use num::integer::lcm;

struct Monkey {
    _name: String,
    business: u64,
    inventory: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    modulo: u64,
    true_index: usize,
    false_index: usize,
}

pub fn solve_part1(input: &str) -> u64 {
    let lines = input.lines();

    let monkeys = parse_input(lines);

    solve(monkeys, 20, Box::new(move |x| x / 3))
}

pub fn solve_part2(input: &str) -> u64 {
    let lines = input.lines();

    let monkeys = parse_input(lines);

    let lcm = monkeys
        .iter()
        .fold(1, |result, monkey| lcm(result, monkey.modulo));

    solve(monkeys, 10_000, Box::new(move |x| x % lcm))
}

fn solve(mut monkeys: Vec<Monkey>, iterations: i32, reduce: Box<dyn Fn(u64) -> u64>) -> u64 {
    for _round in 0..iterations {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].inventory.pop_front() {
                //inspect
                monkeys[i].business += 1;

                let worry_amount = reduce((*monkeys[i].operation)(item));

                //test
                let index = if worry_amount % monkeys[i].modulo == 0 {
                    monkeys[i].true_index
                } else {
                    monkeys[i].false_index
                };

                //throw
                monkeys[index].inventory.push_back(worry_amount);
            }
        }
    }

    let top = monkeys
        .iter()
        .fold((0, 0), |top_two, monkey| match monkey.business {
            x if x > top_two.0 => (x, top_two.0),
            x if x > top_two.1 => (top_two.0, x),
            _ => top_two,
        });

    return top.0 * top.1;
}

fn parse_input(mut lines: Lines) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut line = lines.next();

    while let Some(name) = line {
        let inventory = parse_starting_items(lines.next().unwrap());
        let operation = parse_operation(lines.next().unwrap());
        let modulo = parse_last_number(lines.next().unwrap());
        let true_index = parse_last_number(lines.next().unwrap()) as usize;
        let false_index = parse_last_number(lines.next().unwrap()) as usize;

        let monkey = Monkey {
            _name: name.to_string(),
            business: 0,
            inventory,
            operation,
            modulo,
            true_index,
            false_index,
        };

        monkeys.push(monkey);
        lines.next();
        line = lines.next();
    }

    monkeys
}

fn parse_starting_items(line: &str) -> VecDeque<u64> {
    let mut items = VecDeque::new();
    let item_line = line.split_once(":").unwrap_or_default().1.split(",");

    for item in item_line {
        items.push_back(item.trim().parse().unwrap());
    }

    items
}

fn parse_operation(line: &str) -> Box<dyn Fn(u64) -> u64> {
    let mut expression = line.split_once("=").unwrap().1.split_whitespace();
    expression.next();
    let op = parse_operator(expression.next());
    let right = parse_term(expression.next());

    if let Some(number) = right {
        return Box::new(move |x| op(x, number));
    }
    return Box::new(move |x| op(x, x));
}

fn parse_term(term: Option<&str>) -> Option<u64> {
    match term {
        Some("old") => None,
        Some(x) => Some(x.parse().unwrap()),
        _ => None,
    }
}

fn parse_operator(operation: Option<&str>) -> fn(u64, u64) -> u64 {
    match operation {
        Some("*") => return |a, b| a * b,
        _ => return |a, b| a + b,
    }
}

fn parse_last_number(line: &str) -> u64 {
    line.split_whitespace().last().unwrap().parse().unwrap()
}

#[cfg(test)]
mod day11_tests {
    use crate::solutions::day11;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day11_ex1.txt");
        let result = day11::solve_part1(input);
        assert_eq!(result, 10605)
    }

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day11_ex1.txt");
        let result = day11::solve_part2(input);
        assert_eq!(result, 2713310158)
    }
}
