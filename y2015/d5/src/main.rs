use std::fs;

fn is_nice(s: &str) -> bool {
    if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
        return false;
    }

    let mut vowels: u8 = 0;
    let mut has_double_char: bool = false;

    for (i, c) in s.chars().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u'  => vowels += 1,
            _ => {}
        }
        
        if i + 2 > s.len() {
            continue;
        }
        if s.get(i..i+2).unwrap() == format!("{c}{c}") {
            has_double_char = true;
        }
    }

    if vowels < 3 {
        return false;
    }

    if !has_double_char {
        return false;
    }
    
    true
}

fn is_nice2(s: &str) -> bool {
    let mut pair = s.split_at(2);
    let mut has_double: bool = false;
    let mut has_one_between: bool = false;

    for (i, c) in s.chars().enumerate() {
        if pair.1.contains(pair.0) {
            has_double = true;
        }

        if c == s.chars().nth(i+2).unwrap_or(' ') {
            has_one_between = true;
        }

        if 3 + i > s.len() {
            continue;
        }
        pair = s.split_at(3+i);
        pair.0 = pair.0.split_at(i+1).1;
    }

    if has_double && has_one_between {
        return true;
    }

    false
}

fn main() {
    let mut nices: u16 = 0;
    let mut nices2: u16 = 0;

    for line in fs::read_to_string("/home/t/koodit/Rust/adventOfCode/y2015/d5/src/input.txt").unwrap().lines() {
        if is_nice(line) {
            nices += 1;
        }

        if is_nice2(line) {
            nices2 += 1;
        }
    }

    println!("Part 1: Number of nice people: {}", nices);
    println!("Part 2: Number of nice people: {}", nices2);
}
