use std::{collections::VecDeque, str::Lines};

use itertools::Itertools;

pub fn solve_part1(input: &str) -> String {
    let mut lines = input.lines();
    let boxes = lines.take_while_ref(|line| !line.is_empty()).collect_vec();

    let mut columns = parse_boxes(boxes);

    lines.next();

    columns = parse_moves_part1(lines, columns);

    columns
        .iter_mut()
        .map(|column| column.pop_front().unwrap_or(' '))
        .collect()
}

pub fn solve_part2(input: &str) -> String {
    let mut lines = input.lines();
    let boxes = lines.take_while_ref(|line| !line.is_empty()).collect_vec();
    let mut columns = parse_boxes(boxes);

    lines.next();

    columns = parse_moves_part2(lines, columns);

    columns
        .iter_mut()
        .map(|column| column.pop_front().unwrap_or(' '))
        .collect()
}

fn parse_boxes(mut lines: Vec<&str>) -> Vec<VecDeque<char>> {
    lines.reverse();

    let mut boxlines = lines.iter();
    let num = (boxlines.next().unwrap().len() + 1) / 4;
    let mut columns = Vec::new();

    for _ in 0..num {
        columns.push(VecDeque::<char>::new());
    }

    for boxline in boxlines {
        let mut chars = boxline.chars().skip(1).step_by(4);

        for i in 0..num {
            let char = chars.next();

            match char {
                Some(' ') => continue,
                Some(x) => columns[i].push_front(x),
                _ => continue,
            }
        }
    }

    columns
}

fn parse_moves_part1(move_lines: Lines, mut columns: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    for moves in move_lines {
        let mut chars = moves.split_whitespace();
        let num = chars.nth(1).unwrap().parse::<usize>().unwrap();
        let source = chars.nth(1).unwrap().parse::<usize>().unwrap();
        let goal = chars.nth(1).unwrap().parse::<usize>().unwrap();

        for _ in 0..num {
            let char = columns[source - 1].pop_front();

            match char {
                Some(x) => columns[goal - 1].push_front(x),
                _ => continue,
            }
        }
    }

    columns
}

fn parse_moves_part2(move_lines:Lines, mut columns: Vec<VecDeque<char>>) -> Vec<VecDeque<char>> {
    for moves in move_lines {
        let mut chars = moves.split_whitespace();
        let num = chars.nth(1).unwrap().parse::<usize>().unwrap();
        let source = chars.nth(1).unwrap().parse::<usize>().unwrap();
        let goal = chars.nth(1).unwrap().parse::<usize>().unwrap();

        let mut claw = VecDeque::<char>::new();
        for _ in 0..num {
            let char = columns[source - 1].pop_front();

            match char {
                Some(x) => claw.push_front(x),
                _ => break,
            }
        }

        while claw.len() > 0 {
            match claw.pop_front() {
                Some(x) => columns[goal- 1].push_front(x),
                _ => break,
            }
        }
        
    }

    columns
}

#[cfg(test)]
mod day05_tests {
    use crate::solutions::day05;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day05_ex1.txt");
        let result = day05::solve_part1(input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part1_ex2() {
        let input = include_str!("input/day05_ex2.txt");
        let result = day05::solve_part1(input);
        assert_eq!(result, "FCB");
    }

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day05_ex1.txt");
        let result = day05::solve_part2(input);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn part2_ex2() {
        let input = include_str!("input/day05_ex2.txt");
        let result = day05::solve_part2(input);
        assert_eq!(result, "EAB");
    }
}
