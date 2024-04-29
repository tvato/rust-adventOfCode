use std::fs;
use std::cmp;

fn find_min(pair: (i32,i32,i32)) -> i32 {
    let (l, w, h) = pair;
    let mut min: i32 = l*w;

    if w*h < min {
        min = w*h;
    }

    if l*h < min {
        min = l*h;
    }

    min
}

fn calc_wrapping_paper(pair: (i32, i32, i32)) -> i32 {
    let (l, w, h) = pair;
    let lw: i32 = l*w;
    let wh: i32 = w*h;
    let hl: i32 = h*l;
    let min: i32 = find_min(pair); //if lw < wh { lw } else if wh < hl { wh } else { hl };
    let sum: i32 = (2*lw) + (2*wh) + (2*hl) + min;

    sum
}

fn calc_sum(arr: &Vec<(i32,i32,i32)>) -> i32 {
    let mut sum: i32 = 0;
    for t in arr {
        sum += calc_wrapping_paper(*t);
    }

    sum
}

fn find_smallest(pair: (i32,i32,i32)) -> (i32, i32) {
    let (l,w,h) = pair;
    let mut smallest: (i32, i32) = (0,0);

    if cmp::min(l, w) == l {
        smallest.0 = l;
        if cmp::min(w, h) == w {
            smallest.1 = w;
        }else{
            smallest.1 = h;
        }
    }else{
        smallest.0 = w;
        if cmp::min(l, h) == l {
            smallest.1 = l;
        }else{
            smallest.1 = h;
        }
    }
    

    smallest
}

fn calc_ribbon_box(pair: (i32,i32,i32)) -> i32 {
    let (l,w,h) = pair;
    let two_smallest: (i32,i32) = find_smallest(pair);
    let mut sum: i32 = two_smallest.0 * 2 + two_smallest.1 * 2;
    sum += l*w*h;

    sum
}

fn calc_ribbon(arr: &Vec<(i32,i32,i32)>) -> i32 {
    let mut sum: i32 = 0;
    for t in arr {
        sum += calc_ribbon_box(*t);
    }
    sum
}

fn main() {
    let mut contents: Vec<(i32,i32,i32)> = Vec::new();
    
    for line in fs::read_to_string("/home/t/koodit/Rust/adventOfCode/y2015/d2/src/input.txt").unwrap().lines() {
        let parts = line.split('x');
        let mut part_tuple: (i32, i32, i32) = (0,0,0);
        for (i, part) in parts.enumerate() {
            if i == 0 {
                part_tuple.0 = part.parse::<i32>().unwrap();
            }else if i == 1 {
                part_tuple.1 = part.parse::<i32>().unwrap();
            }else if i == 2 {
                part_tuple.2 = part.parse::<i32>().unwrap();
            }
        }
        contents.push(part_tuple);
    }

    println!("Sum of wrapping paper: {} ft", calc_sum(&contents));
    println!("Sum of ribbon: {} ft", calc_ribbon(&contents));
}
