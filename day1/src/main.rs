use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let data = read_data("data.txt");
    println!("Part 1 = {:?}", larger_than_prev(&data));
    println!("Part 2 = {:?}", window_version(&data));
}

fn larger_than_prev(data: &[i32]) -> i32 {
    let mut count = 0;
    let mut prev = i32::MAX;
    for num in data {
        if num.cmp(&prev) == cmp::Ordering::Greater {
            count += 1
        };
        prev = *num;
    }
    count
}

fn window_version(data: &[i32]) -> i32 {
    let mut count = 0;

    let mut cur: i32;
    let mut window = [data[0], data[1], data[2]];
    let mut prev = window.iter().sum();
    for (index, num) in data.iter().skip(3).enumerate() {
        window[index % 3] = *num;
        cur = window.iter().sum();

        if cur.cmp(&prev) == cmp::Ordering::Greater {
            count += 1;
        }
        prev = cur;
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
