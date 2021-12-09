use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let data = read_data("data.txt");
    println!("{:?}", part1(&data));
}

fn part1(data: &[Vec<u8>]) -> usize {
    let mut result = 0;
    for (i, row) in data.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            let right;
            let left;
            let above;
            let below;
            if j == 0 {
                left = None;
            } else {
                left = data.get(i).unwrap().get(j - 1);
            }
            if i == 0 {
                above = None;
            } else {
                above = if data.get(i - 1).is_none() {
                    None
                } else {
                    Some(&data[i - 1][j])
                };
            }

            right = data.get(i).unwrap().get(j + 1);
            below = if data.get(i + 1).is_none() {
                None
            } else {
                Some(&data[i + 1][j])
            };

            let checks = [left, right, above, below];
            let mut passed = 0;
            for check in checks.iter().flatten() {
                if num >= check {
                    break;
                }
                passed += 1;
            }
            if passed
                == checks
                    .iter()
                    .fold(0, |acc, x| if x.is_none() { acc } else { acc + 1 })
            {
                result += (num + 1) as usize;
            }
        }
    }
    result
}

fn read_data(filename: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).unwrap());

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut nums = Vec::new();
        for char in unwrapped.chars() {
            nums.push(char.to_string().parse().unwrap());
        }
        result.push(nums);
    }
    result
}
