use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day_one() {
    let path = Path::new("src/scripts/inputs/one.txt");

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

pub fn day_two() {
    let path: &Path = Path::new("src/scripts/inputs/two.txt");

    let mut file: File = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open the file: {}", why),
    };

    let mut s: String = String::new();

    let s: String = match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(why) => panic!("couldn't read to string: {}", why),
    };

    fn str_to_vec(s: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
        s.split(",").map(|n| n.trim().parse()).collect()
    }

    let mut s: Vec<i32> = match str_to_vec(&s) {
        Ok(vec) => vec,
        Err(why) => panic!("error parsing string into vec of ints: {}", why),
    };

    fn run_operations(mut codes: Vec<i32>) -> Vec<i32> {
        let mut code_start_index: usize = 0;
        loop {
            let code_slice: &[i32] = &codes[code_start_index..(code_start_index + 4)];

            match code_slice[0] {
                1 => {
                    let input_one_index = code_slice[1] as usize;
                    let input_two_index = code_slice[2] as usize;
                    let save_index = code_slice[3] as usize;

                    let sum: i32 = codes[input_one_index] + codes[input_two_index];
                    codes[save_index] = sum;
                    code_start_index += 4;
                }
                2 => {
                    let input_one_index = code_slice[1] as usize;
                    let input_two_index = code_slice[2] as usize;
                    let save_index = code_slice[3] as usize;

                    let product: i32 = codes[input_one_index] * codes[input_two_index];
                    codes[save_index] = product;
                    code_start_index += 4;
                }
                99 => {
                    println!("99!");
                    break;
                }
                _ => panic!("code started with an invalid number!"),
            }
        }
        codes
    }

    s[1] = 12;
    s[2] = 2;
    println!("{}", run_operations(s)[0]);
}
