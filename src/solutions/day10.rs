pub fn solve_part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut line = lines.next();
    let mut x = 1;
    let mut result = 0;
    let mut buffer = [0, 0];
    let mut step: i32 = 0;
    let mut skip = false;

    while let Some(instruction) = line {
        if !skip {
            if let Some(delta_x) = parse_instruction(instruction) {
                buffer[(step + 1) as usize % 2] = delta_x;
                skip = true;
            }
        } else {
            skip = false;
        }

        if (step - 19) % 40 == 0 {
            result += x * (step + 1);
        }

        x += buffer[step as usize % 2];
        buffer[step as usize % 2] = 0;
        step += 1;
        if !skip {
            line = lines.next();
        }
    }

    if (step - 19) % 40 == 0 {
        result += x * (step - 19);
    }

    result
}

pub fn solve_part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut line = lines.next();
    let mut x = 1;
    let mut buffer = [0, 0];
    let mut step: i32 = 0;
    let mut skip = false;
    let mut result = String::new();

    while let Some(instruction) = line {
        if !skip {
            if let Some(delta_x) = parse_instruction(instruction) {
                buffer[(step + 1) as usize % 2] = delta_x;
                skip = true;
            }
        } else {
            skip = false;
        }

        result.push(get_crt_char(step, x));

        x += buffer[step as usize % 2];
        buffer[step as usize % 2] = 0;
        step += 1;
        if !skip {
            line = lines.next();
        }
        
        if (step) % 40 == 0 {
            result.push('\r');
            result.push('\n');
        }
    }

    result
}

fn get_crt_char(step: i32, x: i32) -> char {
    let crt_loc = step % 40;
    if (crt_loc - x).abs() < 2 {
        '#'
    } else {
        '.'
    }
}

fn parse_instruction(instruction: &str) -> Option<i32> {
    match instruction {
        "noop" => None,
        _ => instruction
            .split_once(" ")
            .and_then(|x| Some(x.1.parse::<i32>().unwrap())),
    }
}

#[cfg(test)]
mod day10_tests {
    use crate::solutions::day10;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day10_ex1.txt");
        let result = day10::solve_part1(input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day10_ex1.txt");
        let result = day10::solve_part2(input);
        let output = include_str!("input/day10_res1.txt");
        assert_eq!(result, output);
    }
}
