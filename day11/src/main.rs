use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut data = read_data("data.txt");
    let mut data2 = data.clone();
    println!("{:?}", part1(&mut data));
    println!("{:?}", part2(&mut data2));
}

fn part2(data: &mut Vec<Vec<u32>>) -> usize {
    let mut i: usize = 1;
    loop {
        let mut step_flash = 0;
        inc_all(data);
        let mut flashed = HashSet::new();
        loop {
            let cur_flashed = flash(data, &mut flashed);
            step_flash += cur_flashed;
            if cur_flashed == 0 {
                break;
            }
        }
        if step_flash == 100 {
            return i;
        }
        clean(data);
        i += 1;
    }
}

fn part1(data: &mut Vec<Vec<u32>>) -> i32 {
    let mut num_flashed = 0;
    for _ in 0..100 {
        inc_all(data);
        let mut flashed = HashSet::new();
        loop {
            let cur_flashed = flash(data, &mut flashed);
            if cur_flashed == 0 {
                break;
            }
            num_flashed += cur_flashed;
        }
        clean(data);
    }
    num_flashed
}

fn clean(data: &mut Vec<Vec<u32>>) {
    for row in data.iter_mut() {
        for num in row.iter_mut() {
            if *num > 10 {
                *num = 0;
            }
        }
    }
}

fn flash(data: &mut Vec<Vec<u32>>, flashed: &mut HashSet<(i32, i32)>) -> i32 {
    let mut cur_flashed = 0;
    for i in 0..10_i32 {
        for j in 0..10_i32 {
            if data[i as usize][j as usize] > 9 && !flashed.contains(&(i, j)) {
                cur_flashed += 1;
                flashed.insert((i, j));
                data[i as usize][j as usize] += 1;
                if i > 0 && j > 0 {
                    data[(i - 1) as usize][(j - 1) as usize] += 1;
                }
                if i > 0 && j + 1 < 10 {
                    data[(i - 1) as usize][(j + 1) as usize] += 1;
                }
                if i > 0 && j >= 0 {
                    data[(i - 1) as usize][(j) as usize] += 1;
                }
                if j > 0 {
                    data[(i) as usize][(j - 1) as usize] += 1;
                }
                if j + 1 < 10 {
                    data[(i) as usize][(j + 1) as usize] += 1;
                }
                if i + 1 < 10 && j > 0 {
                    data[(i + 1) as usize][(j - 1) as usize] += 1;
                }
                if i + 1 < 10 && j >= 0 {
                    data[(i + 1) as usize][(j) as usize] += 1;
                }
                if i + 1 < 10 && j + 1 < 10 {
                    data[(i + 1) as usize][(j + 1) as usize] += 1;
                }
            }
        }
    }
    cur_flashed
}

fn inc_all(data: &mut Vec<Vec<u32>>) {
    for row in data {
        for num in row {
            *num += 1;
        }
    }
}

fn read_data(filename: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).unwrap());
    for line in reader.lines() {
        let mut nums = Vec::new();
        let unwraped = line.unwrap();
        for char in unwraped.chars() {
            nums.push(char.to_digit(10).unwrap());
        }
        result.push(nums);
    }
    result
}
