use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let mut data = read_data("data.txt");
    println!("Part1 {:?}", part1(&mut data));
    println!("Part2 {:?}", part2(&data));
}

fn part2(data: &[Vec<char>]) -> i64 {
    let values: HashMap<char, i64> =
        std::collections::HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut result = Vec::new();
    for line in data {
        let mut stack = Vec::new();
        for char in line {
            match char {
                '(' => stack.push('('),
                '[' => stack.push('['),
                '{' => stack.push('{'),
                '<' => stack.push('<'),
                _ => {
                    stack.pop();
                }
            }
        }

        if !stack.is_empty() {
            let mut score: i64 = 0;
            for symbol in stack.iter().rev() {
                score *= 5;
                score += values.get(symbol).unwrap();
            }
            result.push(score);
        }
    }
    result.sort_unstable();
    result[result.len() / 2]
}

fn part1(data: &mut Vec<Vec<char>>) -> i32 {
    let mut result = Vec::new();
    let mut bad_lines = Vec::new();
    for (index, line) in data.iter().enumerate() {
        let mut stack = Vec::new();
        for char in line {
            match char {
                '(' => stack.push('('),
                '[' => stack.push('['),
                '{' => stack.push('{'),
                '<' => stack.push('<'),
                x => {
                    let cost = handle_close(stack.last().unwrap(), x);
                    if cost == 0 {
                        stack.pop();
                    } else {
                        result.push(cost);
                        bad_lines.push(index);
                        break;
                    }
                }
            }
        }
    }
    for i in bad_lines {
        data[i].clear();
    }
    result.iter().sum()
}

fn handle_close(expected: &char, found: &char) -> i32 {
    let values: HashMap<char, i32> =
        std::collections::HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let pair: HashMap<char, char> =
        std::collections::HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    if pair.get(found).unwrap() == expected {
        return 0;
    }
    *values.get(found).unwrap()
}

fn read_data(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).unwrap());

    for line in reader.lines() {
        let mut chars = Vec::new();
        let unwrapped = line.unwrap();
        for c in unwrapped.chars() {
            chars.push(c);
        }
        result.push(chars);
    }
    result
}
