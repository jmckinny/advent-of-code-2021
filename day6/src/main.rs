use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let data = read_data("data.csv");
    println!("Part1 {:?}", calc_laternfish(&data, 80));
    println!("Part2 {:?}", calc_laternfish(&data, 256))
}

fn calc_laternfish(fishes: &[i32], days: usize) -> i64 {
    let mut states = [0; 9];
    for fish in fishes {
        states[*fish as usize] += 1;
    }
    for _ in 0..days {
        let num_zero = states[0];
        states.rotate_left(1);
        states[6] += num_zero;
        states[8] = num_zero;
    }

    states.iter().sum::<i64>()
}

fn read_data(filename: &str) -> Vec<i32> {
    let mut reader = BufReader::new(File::open(filename).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    nums
}
