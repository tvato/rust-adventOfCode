use std::fs;

fn find_floor(slice: &str) -> i16 {
    let mut floor: i16 = 0;
    for c in slice.chars() {
        if c == '(' {
            floor += 1;
        }else if c == ')' {
            floor -= 1;
        }
    }
    floor
}

fn find_basement_position(slice: &str) -> i16 {
    let mut floor : i16 = 0;
    for (i, c) in slice.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            return i as i16 + 1;
        }
    }

    floor
}

fn main() {
    let contents = fs::read_to_string("/home/t/koodit/Rust/adventOfCode/y2015/d1/src/input.txt").expect("Could not open the file");

    println!("Santa ends up at floor {}", find_floor(&contents));
    println!("Position where Santa first enters basement: {}", find_basement_position(&contents));
}
