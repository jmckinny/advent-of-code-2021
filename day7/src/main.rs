use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut data = read_data("data.csv");
    println!("Part1: {}", part1(&mut data));
    println!("Part2: {}", part2(&data));
}

fn part1(data: &mut [i32]) -> i32 {
    data.sort_unstable();
    let target = data[data.len() / 2];
    calc_fuel(data, target)
}

fn part2(data: &[i32]) -> i32 {
    let target = data.iter().sum::<i32>() / data.len() as i32;
    /*Yes this is stupid but it does work :P */
    let mut close = [0, 0, 0];
    close[0] = calc_fuel_2(data, target - 1);
    close[1] = calc_fuel_2(data, target);
    close[2] = calc_fuel_2(data, target + 1);
    *close.iter().min().unwrap()
}

fn calc_fuel_2(data: &[i32], target: i32) -> i32 {
    let mut fuel = 0;
    for num in data {
        let dif = (*num - target).abs();
        fuel += (dif * (dif + 1)) / 2; //Shoutout to gauss
    }
    fuel
}
fn calc_fuel(data: &[i32], target: i32) -> i32 {
    data.iter().fold(0, |acc, x| acc + ((x - target).abs()))
}

fn read_data(filename: &str) -> Vec<i32> {
    let mut reader = BufReader::new(File::open(filename).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let nums: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    nums
}
