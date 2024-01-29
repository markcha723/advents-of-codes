use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day_one() {
    let path = Path::new("src/scripts/inputs/nineteen.txt");
    println!("day one!");

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open the file: {}", why),
    };

    let mut s = String::new();
    let s: String = match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(why) => panic!("couldn't read to string: {}", why),
    };

    let mut sum = 0;
    for line in s.lines() {
        match line.trim().parse::<i32>() {
            Ok(num) => sum += (num / 3) - 2,
            Err(why) => panic!("invalid input: {}", why),
        };
    }
    println!("{}", sum.to_string());
}
