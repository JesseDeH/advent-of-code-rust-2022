use itertools::Itertools;
use std::cmp;

fn main() {
    let mut input = include_str!("input.txt").lines();
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

    println!("{sum}");
}

fn convert_to_priority(character: char) -> i32 {
    if character.is_uppercase() {
        character as i32 - 38
    } else {
        character as i32 - 96
    }
}
