use itertools::Itertools;

fn main() {
    let grouped_lines = include_str!("input.txt")
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

    let total: i32 = result.iter().sum();

    print!("{total}")
}
