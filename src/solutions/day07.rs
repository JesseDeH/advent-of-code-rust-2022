enum CommandLineEntry<'a> {
    Cd { name: &'a str },
    File { size: usize },
    Other,
}

struct Directory {
    size: usize,
}

fn new_directory() -> Directory {
    Directory {
        size: 0,
    }
}

pub fn solve_part1(input: &str) -> usize {
    let directories = parse_lines(input);

    directories.iter().fold(0, |total, dir| {
        if dir.size <= 100000 {
            total + dir.size
        } else {
            total
        }
    })
}

pub fn solve_part2(input: &str) -> usize {
    let disk_size = 70000000;
    let space_required = 30000000;
    let directories = parse_lines(input);

    if let Some(root) = directories.iter().max_by(|l, r| l.size.cmp(&r.size)) {
        let savings_required = space_required - (disk_size - root.size);
        directories.iter().fold(std::usize::MAX, |cur_size, dir| {
            if dir.size > savings_required && dir.size < cur_size {
                dir.size
            } else {
                cur_size
            }
        })
    } else {
        0
    }
}

fn parse_lines(input: &str) -> Vec<Directory> {
    let mut current: Vec<Directory> = vec![];
    let mut finished: Vec<Directory> = vec![];

    for line in input.lines() {
        let cmd_entry = parse_command_line_entry(line);

        match cmd_entry {
            CommandLineEntry::Cd { name: ".." } => {
                finished.push(current.pop().unwrap());
            }
            CommandLineEntry::Cd { .. } => {
                current.push(new_directory());
            }
            CommandLineEntry::File { size, .. } => {
                for dir in &mut current {
                    dir.size += size;
                }
            }
            _ => {}
        }
    }

    finished.extend(current);
    finished
}

fn parse_command_line_entry(line: &str) -> CommandLineEntry {
    let mut words = line.split_whitespace();
    let first_word = words.next();
    match first_word {
        Some("$") => match words.next() {
            Some("cd") => {
                return CommandLineEntry::Cd {
                    name: words.next().unwrap(),
                }
            }
            _ => return CommandLineEntry::Other,
        },
        Some("dir") => return CommandLineEntry::Other,
        _ => {
            return CommandLineEntry::File {
                size: first_word.unwrap().parse().unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod day07_tests {
    use crate::solutions::day07;

    #[test]
    fn part1_ex1() {
        let input = include_str!("input/day07_ex1.txt");
        let result = day07::solve_part1(input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part2_ex1() {
        let input = include_str!("input/day07_ex1.txt");
        let result = day07::solve_part2(input);
        assert_eq!(result, 24933642);
    }
}
