use std::collections::{HashSet, VecDeque};

type Point = (usize, usize);
type Matrix = Vec<Vec<u32>>;

trait GetExt {
    fn get(&self, point: Point) -> u32;
}

impl GetExt for Matrix {
    fn get(&self, point: Point) -> u32 {
        self[point.1][point.0]
    }
}

pub fn solve_part1(input: &str) -> i32 {
    let (matrix, start, end, _) = parse_input(input);

    solve(&matrix, start, end)
}

pub fn solve_part2(input: &str) -> i32 {
    let (matrix, _, end, low_points) = parse_input(input);

    low_points.into_iter().fold(i32::MAX, |min, start| {
        let length = solve(&matrix, start, end);
        if length < min {
            length
        }else {
            min
        }
    })
}

fn solve(matrix: &Matrix, start: Point, end: Point) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::<Point>::new();
    let max_x = matrix[0].len();
    let max_y = matrix.len();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((cur_loc, round)) = queue.pop_front() {
        let cur_value = matrix.get(cur_loc);
        if cur_loc == end {
            return round;
        }

        for cell in get_adjecent(cur_loc, max_x, max_y) {
            if !visited.contains(&cell) && matrix.get(cell) <= cur_value + 1 {
                visited.insert(cell);
                queue.push_back((cell, round + 1));
            }
        }
    }

    i32::MAX
}

fn get_adjecent((px, py): Point, max_x: usize, max_y: usize) -> Vec<Point> {
    let mut result = Vec::new();

    if px > 0 {
        result.push((px - 1, py));
    }
    if px < max_x - 1 {
        result.push((px + 1, py));
    }
    if py > 0 {
        result.push((px, py - 1));
    }
    if py < max_y - 1 {
        result.push((px, py + 1));
    }

    result
}

fn parse_input(input: &str) -> (Matrix, Point, Point, Vec<Point>) {
    let mut matrix = Vec::new();
    let mut low_points = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut y = 0;

    for line in input.lines() {
        let mut row = Vec::new();
        let mut x = 0;
        for cell in line.chars() {
            if cell == 'a' {
                low_points.push((x,y));
            }
            if cell == 'S' {
                low_points.push((x,y));
                start = (x, y);
                row.push('a' as u32);
            } else if cell == 'E' {
                end = (x, y);
                row.push('z' as u32);
            } else {
                row.push(cell as u32);
            }
            x += 1;
        }
        y += 1;
        matrix.push(row);
    }

    (matrix, start, end, low_points)
}

#[cfg(test)]
mod day11_tests {
    use crate::solutions::day12;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day12_ex1.txt");
        let result = day12::solve_part1(input);
        assert_eq!(result, 31)
    }

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day12_ex1.txt");
        let result = day12::solve_part2(input);
        assert_eq!(result, 29)
    }
}
