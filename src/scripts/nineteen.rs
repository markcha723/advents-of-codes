use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day_one() {
    let path = Path::new("src/scripts/inputs/nineteen.txt");

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open the file: {}", why),
    };

    let mut s = String::new();
    let s: String = match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(why) => panic!("couldn't read to string: {}", why),
    };

    fn calculate_fuel(fuel: i64) -> i64 {
        let mut sum: i64 = 0;
        let mut fuel_for_fuel: i64 = fuel / 3 - 2;
        while fuel_for_fuel > 0 {
            sum += fuel_for_fuel;
            fuel_for_fuel = (fuel_for_fuel / 3) - 2;
        }
        sum
    }

    let mut sum = 0;

    for line in s.lines() {
        match line.trim().parse::<i64>() {
            Ok(num) => sum += calculate_fuel(num),
            Err(why) => panic!("invalid input: {}", why),
        };
    }
    println!("{}", sum.to_string());
}
