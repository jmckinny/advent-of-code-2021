use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let data = read_data("data.txt");
    let result = larger_than_prev(data);
    println!("{:?}", result);
}

fn larger_than_prev(data: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = 0;
    for num in data.into_iter().skip(1) {
        if num.cmp(&prev) == cmp::Ordering::Greater {
            count += 1
        };
        prev = num;
    }
    count
}

fn read_data(filename: &'static str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        result.push(num);
    }
    result
}
