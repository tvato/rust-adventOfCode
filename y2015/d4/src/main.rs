use md5::{Md5, Digest};
use std::time::SystemTime;

fn main() {
    //let key: &str = "yzbqklnj";

    //let mut hasher = Md5::new();
    //let mut num: u64 = 1;

    let start = SystemTime::now();

    for i in 0..u64::MAX {
        let mut hasher = Md5::new();
        hasher.update(format!("yzbqklnj{}", format!("{}", i)));
        let hash = hasher.finalize();

        if format!("{:x}", hash).starts_with("000000") {
            println!("{:x}\n{}", hash, i);
            break;
        }
    }

    println!("Time took: {:?}", start.elapsed().unwrap());
    

    /*
    let mut hasher = Md5::new();

    hasher.update(format!("abcdef{}", format!("{}", 609043)));

    let hash = hasher.finalize();

    //let needle = &[0x0,0x0,0x0,0x0,0x0];

    println!("{:x}", hash);
    println!("{:?}", format!("{:x}", hash).starts_with("00000"));
    */

    //println!("{:x}", hash);
}
