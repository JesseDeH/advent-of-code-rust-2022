use std::collections::HashSet;

pub fn solve_part1(input: &str) -> usize {
    let forest = parse_forest(input);

    let size = forest.len();
    let mut visible = HashSet::new();

    for x in 0..size {
        let mut left_highest = None;
        let mut top_highest = None;
        let mut right_highest = None;
        let mut bottom_highest = None;

        for y in 0..size {
            let left = forest[x][y];
            if left_highest == None || left > left_highest.unwrap_or_default() {
                visible.insert((x, y));
                left_highest = Some(left);
            }
            let top = forest[y][x];
            if top_highest == None || top > top_highest.unwrap_or_default() {
                visible.insert((y, x));
                top_highest = Some(top);
            }
            let right = forest[size - x - 1][size - y - 1];
            if right_highest == None || right > right_highest.unwrap_or_default() {
                visible.insert((size - x - 1, size - y - 1));
                right_highest = Some(right);
            }
            let bottom = forest[size - y - 1][size - x - 1];
            if bottom_highest == None || bottom > bottom_highest.unwrap_or_default() {
                visible.insert((size - y - 1, size - x - 1));
                bottom_highest = Some(bottom);
            }
        }
    }

    visible.len()
}

pub fn solve_part2(input: &str) -> usize {
    let forest = parse_forest(input);

    let mut max = 0;

    let size = forest.len();
    for x in 1..size - 1 {
        for y in 1..size - 1 {
            let score = get_scenic_score(x, y, &forest);
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn parse_forest(input: &str) -> Vec<Vec<u32>> {
    let mut forest: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let mut treeline = Vec::new();
        for char_digit in line.chars() {
            treeline.push(char_digit.to_digit(10).unwrap());
        }
        forest.push(treeline);
    }

    forest
}

fn get_scenic_score(tree_x: usize, tree_y: usize, forest: &Vec<Vec<u32>>) -> usize {
    let size = forest.len();
    let left_score = count_direction((tree_x, tree_y), size, &forest, |(x, y)| (x - 1, y));
    let right_score = count_direction((tree_x, tree_y), size, &forest, |(x, y)| (x + 1, y));
    let top_score = count_direction((tree_x, tree_y), size, &forest, |(x, y)| (x, y - 1));
    let bottom_score = count_direction((tree_x, tree_y), size, &forest, |(x, y)| (x, y + 1));

    left_score * right_score * top_score * bottom_score
}

fn count_direction(
    loc: (usize, usize),
    size: usize,
    forest: &Vec<Vec<u32>>,
    func: fn((i32, i32)) -> (i32, i32),
) -> usize {
    let edge_found: bool = false;
    let mut count = 0;
    let height = forest[loc.0][loc.1];
    let (mut x, mut y) = func((i32::try_from(loc.0).unwrap(), i32::try_from(loc.1).unwrap()));
    while !edge_found {
        if x < 0 || y < 0 || x >= size.try_into().unwrap() || y >= size.try_into().unwrap()
        {
            break;
        }

        count += 1;

        if x >= 0 && y >= 0 && x < size.try_into().unwrap() && y < size.try_into().unwrap() && height <= forest[x as usize][y as usize] {
            break;
        }

        (x, y) = func((x, y));
    }

    count
}

#[cfg(test)]
mod day08_tests {
    use crate::solutions::day08;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day08_ex1.txt");
        let result = day08::solve_part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day08_ex1.txt");
        let result = day08::solve_part2(input);
        assert_eq!(result, 8);
    }
}
