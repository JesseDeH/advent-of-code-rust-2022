use itertools::Itertools;
use std::cmp;

pub fn solve_part1(input: &str) -> i32 {
    let input = input.lines();
    let mut duplicates = Vec::new();

    for line in input {
        let half = line.len() / 2;
        let splits = line.split_at(half);
        for char_left in splits.0.chars() {
            let mut found = false;
            for char_right in splits.1.chars() {
                if char_left == char_right {
                    found = true;
                    duplicates.push(convert_to_priority(char_left));
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    duplicates.iter().sum()
}

pub fn solve_part2(raw_input: &str) -> i32 {
    let mut input = raw_input.lines();
    let mut sum = 0;

    let mut line1 = input.next();
    let mut line2 = input.next();
    let mut line3 = input.next();

    while line1 != None {
        let mut inv1 = line1.expect("Input not multiple of three").chars().sorted();
        let mut inv2 = line2.expect("Input not multiple of three").chars().sorted();
        let mut inv3 = line3.expect("Input not multiple of three").chars().sorted();

        let mut item1 = inv1.next();
        let mut item2 = inv2.next();
        let mut item3 = inv3.next();

        while item1 != None {
            if item1 == item2 && item2 == item3 {
                sum += convert_to_priority(item1.expect("what"));
                break;
            }
            let min = cmp::min(cmp::min(item1, item2), item3);
            if item1 == min {
                item1 = inv1.next();
            }
            if item2 == min {
                item2 = inv2.next();
            }
            if item3 == min {
                item3 = inv3.next();
            }
        }

        line1 = input.next();
        line2 = input.next();
        line3 = input.next();
    }
    
    sum
}

fn convert_to_priority(character: char) -> i32{
    if character.is_uppercase(){
        character as i32 - 38
    } else {
        character as i32 - 96
    }
}