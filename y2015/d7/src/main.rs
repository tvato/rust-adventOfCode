use std::{collections::HashMap, fs};

fn construct_hashmap(v: Vec<String>) -> HashMap<String, u16> {
    let mut res: HashMap<String, u16> = HashMap::new();
    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        if split[0] == "NOT" {
            res.entry(String::from(split[1])).or_insert(0);
            res.entry(String::from(split[3])).or_insert(0);
        }else if split[1] == "->" {
            if split[0].parse::<u16>().is_err() {
                res.entry(String::from(split[0])).or_insert(0);
            }
            res.entry(String::from(split[2])).or_insert(0);
        }else if split[1] == "AND" {
            res.entry(String::from(split[0])).or_insert(0);
            res.entry(String::from(split[2])).or_insert(0);
            res.entry(String::from(split[4])).or_insert(0);
        }else if split[1] == "RSHIFT" {
            res.entry(String::from(split[0])).or_insert(0);
            res.entry(String::from(split[4])).or_insert(0);
        }else if split[1] == "LSHIFT" {
            res.entry(String::from(split[0])).or_insert(0);
            res.entry(String::from(split[4])).or_insert(0);
        }else if split[1] == "OR" {
            res.entry(String::from(split[0])).or_insert(0);
            res.entry(String::from(split[2])).or_insert(0);
            res.entry(String::from(split[4])).or_insert(0);
        }
    }

    res
}

fn calc_NOTs(num: u16) -> u16 {
    !num
}

fn find_NOTs(v: Vec<String>, h: &mut HashMap<String, u16>) {
    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        if s.contains("NOT") {
            //h.entry(split.last().unwrap().to_string()).or_insert(calc_NOTs(split[1].parse::<u16>().unwrap()));
            h.insert(split.last().unwrap().to_string(), calc_NOTs(split[1].parse::<u16>().unwrap()));
        }
    }
}

fn calc_ANDs(a: u16, b: u16) -> u16 {
    a & b
}

fn find_ANDs(v: Vec<String>, h: &mut HashMap<String, u16>){
    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        if s.contains("AND") {
            h.insert(split.last().unwrap().to_string(), calc_ANDs(split[0].parse::<u16>().unwrap(), split[2].parse::<u16>().unwrap()));
        }
    }
}

fn calc_ORs(a: u16, b: u16) -> u16 {
    a | b
}

fn find_ORs(v: Vec<String>, h: &mut HashMap<String, u16>){
    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        if s.contains("OR") {
            h.insert(split.last().unwrap().to_string(), calc_ORs(split[0].parse::<u16>().unwrap(), split[2].parse::<u16>().unwrap()));
        }
    }
}

fn calc_RSHIFTs(a: u16, b: u16) -> u16 {
    a >> b
}

fn find_RSHIFTs(v: Vec<String>, h: &mut HashMap<String, u16>){
    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        if s.contains("RSHIFT") {
            h.insert(split.last().unwrap().to_string(), calc_RSHIFTs(split[0].parse::<u16>().unwrap(), split));
        }
    }
}

/*
fn parse_and_calc(v: Vec<String>) -> HashMap<String, u16> {
    let mut res: HashMap<String, u16> = HashMap::new();
    for s in v {
        let s_v: Vec<&str> = s.split(' ').collect();
        if s_v[1] == "->" {
            if s_v[0].parse::<u16>().is_ok() {
                res.entry(s_v.last().unwrap().to_string()).or_insert(s_v.first().unwrap().parse::<u16>().unwrap());
            }else{
                let r = *res.entry(s_v.first().unwrap().to_string()).or_insert(0);
                res.entry(s_v.last().unwrap().to_string()).or_insert(r);
            }
        }else if s_v[1] == "AND" {
            let r = *res.entry(String::from(s_v[0])).or_insert(0) & *res.entry(String::from(s_v[2])).or_insert(0);
            res.entry(s_v.last().unwrap().to_string()).or_insert(r);
        }else if s_v[1] == "OR" {
            let r = *res.entry(String::from(s_v[0])).or_insert(0) | *res.entry(String::from(s_v[2])).or_insert(0);
            res.entry(s_v.last().unwrap().to_string()).or_insert(r);
        }else if s_v[1] == "LSHIFT" {
            let r = *res.entry(String::from(s_v[0])).or_insert(0) << s_v[2].parse::<u16>().unwrap();
            res.entry(s_v.last().unwrap().to_string()).or_insert(r);
        }else if s_v[1] == "RSHIFT" {
            let r = *res.entry(String::from(s_v[0])).or_insert(0) >> s_v[2].parse::<u16>().unwrap();
            println!("s: {}, r: {}", s, r);
            res.entry(s_v.last().unwrap().to_string()).or_insert(r);
        }else if s_v[0] == "NOT" {
            let r = !*res.entry(String::from(s_v[1])).or_insert(0);
            res.entry(s_v.last().unwrap().to_string()).or_insert(r);
        }
    }

    res
}
*/

fn main() {
    let inst: Vec<String> = fs::read_to_string("/home/t/koodit/Rust/adventOfCode/y2015/d7/src/input.txt").unwrap().lines().map(String::from).collect();
    //println!("{:?}", inst);
    //let map: HashMap<String, u16> = parse_and_calc(inst);
    let mut map: HashMap<String, u16> = construct_hashmap(inst);
    println!("{:?}", map);
}
