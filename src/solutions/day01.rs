use itertools::Itertools;
use std::cmp;

pub fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut total = 0;
    let mut max = 0;

    for line in lines {
        if line.is_empty() {
            max = cmp::max(max, total);
            total = 0;
            continue;
        }
        let weight = line.parse::<i32>().expect("line not parseable");
        total += weight;
    }

    total
}


pub fn solve_part2(input: &str) -> i32 {
    let grouped_lines = input
        .lines()
        .group_by(|line| line.is_empty());

    let inventories = grouped_lines
        .into_iter()
        .filter_map(|(key, group)| {
            (!key).then_some(group.map(|x| x.parse::<i32>().expect("Should be a number")))
        })
        .map(|inv| inv.sum::<i32>());

    let mut result = vec![0; 3];
    for weight in inventories {
        if weight > result[0] {
            result[2] = result[1];
            result[1] = result[0];
            result[0] = weight;
        } else if weight > result[1] {
            result[2] = result[1];
            result[1] = weight;
        } else if weight > result[2] {
            result[2] = weight;
        }
    }

   result.iter().sum()
}
