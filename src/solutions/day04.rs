pub fn solve_part1(input: &str) -> i32 {
    input.lines().fold(0, |count, line| {
        let (left_low, left_high, right_low, right_high) = split_line(line);

        if (left_low <= right_low && left_high >= right_high)
            || (right_low <= left_low && right_high >= left_high)
        {
            count + 1
        } else {
            count
        }
    })
}

pub fn solve_part2(input: &str) -> i32 {
    input.lines().fold(0, |count, line| {
        let (left_low, left_high, right_low, right_high) = split_line(line);

        if left_low <= right_low && left_high >= right_low {
            count + 1
        } else if right_low <= left_low && right_high >= left_low {
            count + 1
        } else {
            count
        }
    })
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

#[cfg(test)]
mod tests {
    use crate::solutions::day04;

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day04_ex1.txt");
        let result = day04::solve_part2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_ex2() {
        let input = include_str!("input/day04_ex2.txt");
        let result = day04::solve_part2(input);
        assert_eq!(result, 4);
    }
}
