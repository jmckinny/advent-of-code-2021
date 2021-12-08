use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let data = read_data("data.txt");
    println!("Part1 {:?}", part1(&data));
    println!("Part2 {:?}", part2(&data));
}

fn part2(data: &[Vec<Vec<String>>]) -> i32 {
    let mut result: Vec<String> = Vec::new();
    for line in data {
        let mut one = String::from("");
        let mut four = String::from("");
        let input = line.get(0).unwrap();
        for str in input {
            match str.len() {
                2 => one = str.to_string(),
                4 => four = str.to_string(),
                _ => {}
            }
        }
        let mut decoded = String::from("");
        for str in line.get(1).unwrap() {
            decoded.push_str(to_num(str, common_with(str, &one), common_with(str, &four)).as_str());
        }
        result.push(decoded);
    }

    result.iter().map(|s| s.parse::<i32>().unwrap()).sum()
}

fn common_with(str: &str, num: &str) -> usize {
    num.chars()
        .fold(0, |acc, c| if str.contains(c) { acc + 1 } else { acc })
}

fn to_num(str: &str, common1: usize, common4: usize) -> String {
    match (str.len(), common1, common4) {
        (2, 2, 2) => "1".to_string(),
        (5, 1, 2) => "2".to_string(),
        (5, 2, 3) => "3".to_string(),
        (4, 2, 4) => "4".to_string(),
        (5, 1, 3) => "5".to_string(),
        (6, 1, 3) => "6".to_string(),
        (3, 2, 2) => "7".to_string(),
        (7, 2, 4) => "8".to_string(),
        (6, 2, 4) => "9".to_string(),
        (6, 2, 3) => "0".to_string(),
        _ => panic!("Invalid len!"),
    }
}

fn part1(data: &[Vec<Vec<String>>]) -> i32 {
    let mut output = 0;
    for line in data {
        let outputs = line.get(1).unwrap();
        for str in outputs {
            if is_1_4_7_8(str) {
                output += 1;
            }
        }
    }
    output
}

fn is_1_4_7_8(str: &str) -> bool {
    let mut count = [0; 7]; // a b c d e f g
    for char in str.chars() {
        match char {
            'a' => count[0] += 1,
            'b' => count[1] += 1,
            'c' => count[2] += 1,
            'd' => count[3] += 1,
            'e' => count[4] += 1,
            'f' => count[5] += 1,
            'g' => count[6] += 1,
            _ => panic!("Invalid letter!"),
        }
    }
    let sum: i32 = count.iter().sum();
    sum == 2 || sum == 4 || sum == 3 || sum == 7
}

fn read_data(filename: &str) -> Vec<Vec<Vec<String>>> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).unwrap());
    for line in reader.lines() {
        let unrwaped_line = line.unwrap();
        let mut line_vec = Vec::new();
        for segment in unrwaped_line.split('|') {
            line_vec.push(segment.trim().split(' ').map(|s| s.to_string()).collect());
        }
        result.push(line_vec);
    }
    result
}
