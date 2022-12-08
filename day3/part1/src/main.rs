fn main() {
    let input = include_str!("input.txt").lines();
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

    let result:i32 = duplicates.iter().sum();

    println!("{result}");
}

fn convert_to_priority(character: char) -> i32{
    if character.is_uppercase(){
        character as i32 - 38
    } else {
        character as i32 - 96
    }
}