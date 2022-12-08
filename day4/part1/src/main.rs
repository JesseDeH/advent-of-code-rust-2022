fn main() {
    let input = include_str!("input.txt").lines();

    let mut count = 0;
    
    for line in input {
        let (left_low, left_high, right_low, right_high) = split_line(line);

        if (left_low <= right_low && left_high >= right_high)
            || (right_low <= left_low && right_high >= left_high)
        {
            count += 1;
        }
    }

    println!("{count}");
}

fn split_line(line: &str) -> (i32, i32, i32, i32) {
    let (left, right) = line.split_once(",").expect("a line to have a comma");
    let (left_low, left_high) = left.split_once("-").expect("each part to have a dash");
    let (right_low, right_high) = right.split_once("-").expect("each part to have a dash");

    (
        left_low.parse().unwrap_or_default(),
        left_high.parse().unwrap_or_default(),
        right_low.parse().unwrap_or_default(),
        right_high.parse().unwrap_or_default(),
    )
}
