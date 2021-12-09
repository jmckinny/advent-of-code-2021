use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let data = read_data("data.txt");
    println!("Part1 {:?}", part1(&data));
    println!("Part2 {:?}", part2(&data));
}

fn part2(data: &[Vec<u8>]) -> usize {
    let lows = find_low(data);
    let mut basins: Vec<usize> = lows
        .iter()
        .map(|(row, col)| do_dfs(data, *row, *col))
        .collect();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn do_dfs(data: &[Vec<u8>], row: usize, col: usize) -> usize {
    fn helper(data: &[Vec<u8>], row: usize, col: usize, seen: &mut Vec<(usize, usize)>) -> usize {
        if seen.contains(&(row, col)) || row > data.len() - 1 || col > data[0].len() - 1 {
            return 0;
        }
        seen.push((row, col));
        if data[row][col] == 9 {
            return 0;
        }
        let mut result = 1;
        if row != 0 {
            result += helper(data, row - 1, col, seen);
        }
        if col != 0 {
            result += helper(data, row, col - 1, seen);
        }

        result += helper(data, row + 1, col, seen);
        result += helper(data, row, col + 1, seen);

        result
    }
    helper(data, row, col, &mut Vec::new())
}

fn find_low(data: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
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
                result.push((i, j));
            }
        }
    }
    result
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
