use std::fs;

#[derive(Copy, Clone, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

fn check_visit(visited: &Vec<Pos>, current: &Pos) -> bool {
    if visited.contains(current){
        return true;
    }

    false
}

fn calc_houses(instructions: &str) -> i32 {
    let mut visited_houses: i32 = 1;
    let mut current_pos: Pos = Pos { x: 0, y: 0 };
    let mut visited_pos: Vec<Pos> = Vec::new();
    visited_pos.push(current_pos);

    for c in instructions.chars() {
        if c == '^' {
            current_pos.y += 1;

            if !check_visit(&visited_pos, &current_pos) {
                visited_pos.push(current_pos);
                visited_houses += 1;
            }
        }else if c == '>' {
            current_pos.x += 1;

            if !check_visit(&visited_pos, &current_pos) {
                visited_pos.push(current_pos);
                visited_houses += 1;
            }
        }else if c == '<' {
            current_pos.x -= 1;

            if !check_visit(&visited_pos, &current_pos) {
                visited_pos.push(current_pos);
                visited_houses += 1;
            }
        }else if c == 'v' {
            current_pos.y -= 1;

            if !check_visit(&visited_pos, &current_pos) {
                visited_pos.push(current_pos);
                visited_houses += 1;
            }
        }
    }

    visited_houses
}

fn calc_houses2(instructions: &str) -> i32 {
    let mut visited_houses: i32 = 1;
    let mut santas_current_pos: Pos = Pos { x: 0, y: 0 };
    let mut robots_current_pos: Pos = Pos { x: 0, y: 0 };
    let mut visited_pos: Vec<Pos> = Vec::new();

    visited_pos.push(santas_current_pos);

    let mut turn: bool = true;

    for c in instructions.chars() {
        if c == '^' {
            if turn {
                santas_current_pos.y += 1;
                turn = false;
                if !check_visit(&visited_pos, &santas_current_pos) {
                    visited_pos.push(santas_current_pos);
                    visited_houses += 1;
                }
            }else{
                robots_current_pos.y += 1;
                turn = true;
                if !check_visit(&visited_pos, &robots_current_pos) {
                    visited_pos.push(robots_current_pos);
                    visited_houses += 1;
                }
            }
        }else if c == '>' {
            if turn {
                santas_current_pos.x += 1;
                turn = false;
                if !check_visit(&visited_pos, &santas_current_pos) {
                    visited_pos.push(santas_current_pos);
                    visited_houses += 1;
                }
            }else{
                robots_current_pos.x += 1;
                turn = true;
                if !check_visit(&visited_pos, &robots_current_pos) {
                    visited_pos.push(robots_current_pos);
                    visited_houses += 1;
                }
            }
        }else if c == '<' {
            if turn {
                santas_current_pos.x -= 1;
                turn = false;
                if !check_visit(&visited_pos, &santas_current_pos) {
                    visited_pos.push(santas_current_pos);
                    visited_houses += 1;
                }
            }else{
                robots_current_pos.x -= 1;
                turn = true;
                if !check_visit(&visited_pos, &robots_current_pos) {
                    visited_pos.push(robots_current_pos);
                    visited_houses += 1;
                }
            }
        }else if c == 'v' {
            if turn {
                santas_current_pos.y -= 1;
                turn = false;
                if !check_visit(&visited_pos, &santas_current_pos) {
                    visited_pos.push(santas_current_pos);
                    visited_houses += 1;
                }
            }else{
                robots_current_pos.y -= 1;
                turn = true;
                if !check_visit(&visited_pos, &robots_current_pos) {
                    visited_pos.push(robots_current_pos);
                    visited_houses += 1;
                }
            }
        }
    }

    visited_houses
}

fn main() {
    let contents = fs::read_to_string("/home/t/koodit/Rust/adventOfCode/y2015/d3/src/input.txt").unwrap();
    
    println!("{}", calc_houses(&contents)); // Part 1
    println!("{}", calc_houses2(&contents)); // Part 2
    
}
