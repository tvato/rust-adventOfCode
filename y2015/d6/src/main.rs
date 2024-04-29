use std::fs;

fn turn_off(mut arr: Vec<Vec<bool>>, instruction: &str) -> Vec<Vec<bool>> {
    let chunks: Vec<&str> = instruction.split(" ").collect();
    let from_coords: Vec<&str> = chunks[2].split(",").collect();
    let to_coords: Vec<&str> = chunks[4].split(",").collect();

    for x in from_coords[0].parse().unwrap_or(0)..to_coords[0].parse().unwrap_or(0) + 1{
        for y in from_coords[1].parse().unwrap_or(0)..to_coords[1].parse().unwrap_or(0) + 1 {
            arr[x][y] = false;
        }
    }

    arr
}

fn turn_on(mut arr: Vec<Vec<bool>>, instruction: &str) -> Vec<Vec<bool>>{
    let chunks: Vec<&str> = instruction.split(" ").collect();
    let from_coords: Vec<&str> = chunks[2].split(",").collect();
    let to_coords: Vec<&str> = chunks[4].split(",").collect();

    for x in from_coords[0].parse().unwrap_or(0)..to_coords[0].parse().unwrap_or(0) + 1 {
        for y in from_coords[1].parse().unwrap_or(0)..to_coords[1].parse().unwrap_or(0) + 1 {
            arr[x][y] = true;
        }
    }

    arr
}

fn toggle(mut arr: Vec<Vec<bool>>, instruction: &str) -> Vec<Vec<bool>> {
    let chunks: Vec<&str> = instruction.split(" ").collect();
    let from_coords: Vec<&str> = chunks[1].split(",").collect();
    let to_coords: Vec<&str> = chunks[3].split(",").collect();

    for x in from_coords[0].parse().unwrap_or(0)..to_coords[0].parse().unwrap_or(0) + 1 {
        for y in from_coords[1].parse().unwrap_or(0)..to_coords[1].parse().unwrap_or(0) + 1 {
            if arr[x][y] {
                arr[x][y] = false;
            }else{
                arr[x][y] = true;
            }
        }
    }

    arr
}

fn check_lits(arr: Vec<Vec<bool>>) -> u32 {
    let mut count: u32 = 0;
    for line in arr {
        for coord in line   {
            if coord {
                count += 1;
            }
        }
    }

    count
}

fn lights(instructions: &Vec<String>) -> u32 {
    let mut arr: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    
    for line in instructions {
        if line.contains("turn off") {
            arr = turn_off(arr, &line);
        }else if line.contains("turn on") {
            arr = turn_on(arr, &line);
        }else if line.contains("toggle") {
            arr = toggle(arr, &line);
        }else{
            panic!("HELP! Something went wrong!");
        }
    }

    check_lits(arr)
}

fn turn_off2(mut arr: Vec<Vec<u8>>, instruction: &str) -> Vec<Vec<u8>> {
    let chunks: Vec<&str> = instruction.split(" ").collect();
    let from_coords: Vec<&str> = chunks[2].split(",").collect();
    let to_coords: Vec<&str> = chunks[4].split(",").collect();

    for x in from_coords[0].parse().unwrap_or(0)..to_coords[0].parse().unwrap_or(0) + 1{
        for y in from_coords[1].parse().unwrap_or(0)..to_coords[1].parse().unwrap_or(0) + 1 {
            if arr[x][y] != 0 {
                arr[x][y] -= 1;
            }
        }
    }

    arr
}

fn turn_on2(mut arr: Vec<Vec<u8>>, instruction: &str) -> Vec<Vec<u8>>{
    let chunks: Vec<&str> = instruction.split(" ").collect();
    let from_coords: Vec<&str> = chunks[2].split(",").collect();
    let to_coords: Vec<&str> = chunks[4].split(",").collect();

    for x in from_coords[0].parse().unwrap_or(0)..to_coords[0].parse().unwrap_or(0) + 1 {
        for y in from_coords[1].parse().unwrap_or(0)..to_coords[1].parse().unwrap_or(0) + 1 {
            arr[x][y] += 1;
        }
    }

    arr
}

fn toggle2(mut arr: Vec<Vec<u8>>, instruction: &str) -> Vec<Vec<u8>> {
    let chunks: Vec<&str> = instruction.split(" ").collect();
    let from_coords: Vec<&str> = chunks[1].split(",").collect();
    let to_coords: Vec<&str> = chunks[3].split(",").collect();

    for x in from_coords[0].parse().unwrap_or(0)..to_coords[0].parse().unwrap_or(0) + 1 {
        for y in from_coords[1].parse().unwrap_or(0)..to_coords[1].parse().unwrap_or(0) + 1 {
            arr[x][y] += 2;
        }
    }

    arr
}

fn check_lits2(arr: Vec<Vec<u8>>) -> u32 {
    let mut count: u32 = 0;
    for line in arr {
        for coord in line   {
            count += coord as u32;
        }
    }

    count
}

fn lights2(instructions: &Vec<String>) -> u32 {
    let mut arr: Vec<Vec<u8>> = vec![vec![0; 1000]; 1000];
    
    for line in instructions {
        if line.contains("turn off") {
            arr = turn_off2(arr, &line);
        }else if line.contains("turn on") {
            arr = turn_on2(arr, &line);
        }else if line.contains("toggle") {
            arr = toggle2(arr, &line);
        }else{
            panic!("HELP! Something went wrong!");
        }
    }

    check_lits2(arr)
}

fn main() {
    // For testing purposes
    /*let inst = [String::from("turn off 660,55 through 986,197"),
                            String::from("turn on 0,0 through 999,999")
                            String::from("turn off 199,133 through 461,193"),
                            String::from("toggle 322,558 through 977,958"),
                            String::from("toggle 537,781 through 687,941")
                            ].to_vec();*/

    let inst: Vec<String> = fs::read_to_string("/home/t/koodit/Rust/adventOfCode/y2015/d6/src/input.txt").unwrap().lines().map(String::from).collect();

    println!("{}", lights(&inst));
    println!("{}", lights2(&inst));
}
