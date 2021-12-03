use std::fs::File;
use std::io::{prelude::*, BufReader};
fn main() {
    let data = read_data("data.txt");
    println!("Power {:?}", sub_power(&data));
    println!("Life Support {:?}", life_support(&data));
}

fn sub_power(data: &[String]) -> i32 {
    let mut gama: [i32; 12] = [0; 12];
    for num in data {
        for (index, digit) in num.chars().enumerate() {
            match digit {
                '0' => gama[index] -= 1,
                '1' => gama[index] += 1,
                _ => panic!("invalid char!"),
            }
        }
    }

    let gama_string = to_string(&gama);
    let epsilon = invert(&gama_string);
    let gama_val = isize::from_str_radix(&gama_string, 2).unwrap() as i32;
    let epsilon_val = isize::from_str_radix(&epsilon, 2).unwrap() as i32;
    gama_val * epsilon_val
}

fn bit_criteria(data: &[String], pos: usize, func: fn(&[String], usize) -> char) -> String {
    if data.len() == 1 {
        return data[0].clone();
    }
    let mut remaining: Vec<String> = Vec::new();
    let bit = func(data, pos);
    for num in data {
        if num.chars().nth(pos).unwrap() == bit {
            remaining.push(num.clone());
        }
    }

    bit_criteria(&remaining, pos + 1, func)
}

fn life_support(data: &[String]) -> i32 {
    let oxygen = bit_criteria(data, 0, most_common_bit);
    let co2 = bit_criteria(data, 0, least_common_bit);

    let oxygen_val = isize::from_str_radix(&oxygen, 2).unwrap() as i32;
    let co2_val = isize::from_str_radix(&co2, 2).unwrap() as i32;

    oxygen_val * co2_val
}

fn most_common_bit(data: &[String], pos: usize) -> char {
    let mut count = 0;
    for num in data {
        if num.chars().nth(pos).unwrap() == '1' {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count >= 0 {
        '1'
    } else {
        '0'
    }
}

fn least_common_bit(data: &[String], pos: usize) -> char {
    let mut count = 0;
    for num in data {
        if num.chars().nth(pos).unwrap() == '1' {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count >= 0 {
        '0'
    } else {
        '1'
    }
}

fn to_string(arr: &[i32]) -> String {
    let mut result = String::from("");
    for num in arr {
        if *num > 0 {
            result += "1";
        } else {
            result += "0";
        }
    }
    result
}

fn invert(string: &str) -> String {
    let mut result = String::from("");
    for char in string.chars() {
        match char {
            '0' => result += "1",
            '1' => result += "0",
            _ => panic!("invalid char!"),
        };
    }
    result
}

fn read_data(filename: &'static str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let unwraped_line = line.unwrap();
        result.push(unwraped_line);
    }
    result
}
