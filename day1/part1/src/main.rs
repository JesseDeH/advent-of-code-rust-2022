use std::cmp;

fn main() {
    let lines = include_str!("input.txt").lines();

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

    print!("{max}");
}
