use core::hash::Hash;
use std::collections::HashSet;

pub fn solve_part1(input: &str) -> usize {
    solve(input, 4)
}

pub fn solve_part2(input: &str) -> usize {
    solve(input, 14)
}

fn solve(input: &str, num: usize) -> usize {
    let mut chars = input.chars();
    let mut steps_total = num;

    let mut current_chars = Vec::new();

    for _ in 0..num {
        current_chars.push(chars.next())
    }

    while !all_unique_elements(current_chars.clone()) {
        let index = steps_total % num;
        current_chars[index] = chars.next();
        if current_chars[index] == None {
            panic!("Ran out of characters");
        }
        steps_total += 1;
    }

    steps_total
}

fn all_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod day06_tests {
    use crate::solutions::day06;

    #[test]
    fn part1_ex1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = day06::solve_part1(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_ex2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = day06::solve_part1(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn part1_ex3() {
        let input = "vwbjplbgvbhsrlpgdmjqwftvncz";
        let result = day06::solve_part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_ex4() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = day06::solve_part1(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part1_ex5() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = day06::solve_part1(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn part1_ex6() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = day06::solve_part1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn part2_ex1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = day06::solve_part2(input);
        assert_eq!(result, 19);
    }

    #[test]
    fn part2_ex2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = day06::solve_part2(input);
        assert_eq!(result, 23);
    }

    #[test]
    fn part2_ex3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = day06::solve_part2(input);
        assert_eq!(result, 23);
    }

    #[test]
    fn part2_ex4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = day06::solve_part2(input);
        assert_eq!(result, 29);
    }

    #[test]
    fn part2_ex5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = day06::solve_part2(input);
        assert_eq!(result, 26);
    }
}
