use std::collections::HashSet;

type Vector = (i32, i32);

pub fn solve_part1(input: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut head_location = (0, 0);
    let mut tail_location = (0, 0);
    visited.insert(tail_location);

    for line in input.lines() {
        if let Some((move_step, move_amount)) = line.split_once(" ") {
            let move_step = parse_move(move_step);
            let move_amount = str::parse::<i32>(move_amount).unwrap();

            for _ in 0..move_amount {
                head_location = add(head_location, move_step);
                tail_location = add(tail_location, check_tail_move(head_location, tail_location));

                visited.insert(tail_location);
            }
        }
    }

    visited.len().try_into().unwrap()
}

fn check_tail_move(head_location: Vector, tail_location: Vector) -> Vector {
    let delta = sub(head_location, tail_location);
    match delta {
        (d_x, d_y) if d_x.abs() >= 2 || d_y.abs() >= 2 => (
            d_x.checked_div(d_x.abs()).unwrap_or(0),
            d_y.checked_div(d_y.abs()).unwrap_or(0),
        ),
        _ => (0, 0),
    }
}

fn parse_move(move_step: &str) -> Vector {
    match move_step {
        "L" => (1, 0),
        "U" => (0, 1),
        "R" => (-1, 0),
        "D" => (0, -1),
        _ => (0, 0),
    }
}

fn add((a_x, a_y): Vector, (b_x, b_y): Vector) -> Vector {
    (a_x + b_x, a_y + b_y)
}

fn sub((a_x, a_y): Vector, (b_x, b_y): Vector) -> Vector {
    (a_x - b_x, a_y - b_y)
}

#[cfg(test)]
mod day09_tests {
    use crate::solutions::day09;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day09_ex1.txt");
        let result = day09::solve_part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_ex2() {
        let input = include_str!("input/day09_ex2.txt");
        let result = day09::solve_part1(input);
        assert_eq!(result, 10);
    }
}
