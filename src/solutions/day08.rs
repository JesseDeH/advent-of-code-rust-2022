use nalgebra::DMatrix;

fn solve_part1(input: &str) -> i32{

 0
}

fn parse_matrix(input: &str) -> nalgebra::DMatrix<i32>{

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

}