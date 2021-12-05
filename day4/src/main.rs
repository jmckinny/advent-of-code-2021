use std::{
    fs::File,
    io::{BufRead, BufReader},
    panic,
};

use board::Board;
mod board;

fn main() {
    let nums = read_nums("nums.csv");
    let mut boards = read_boards("boards.txt");
    println!("Part 1: {}", do_bingo(&mut boards, &nums));
    for board in boards.iter_mut() {
        board.reset();
    }
    println!("Part 2: {}", lose_bingo(&mut boards, &nums));
}

fn lose_bingo(boards: &mut Vec<Board>, nums: &[i32]) -> i32 {
    let mut active_boards = boards.len() as i32;
    for num in nums {
        for board in boards.iter_mut() {
            if board.get_score() != 0 {
                continue;
            }
            board.mark(*num);
            if board.is_winner() {
                board.calc_score(*num);
                active_boards -= 1;
                if active_boards == 0 {
                    return board.get_score();
                }
            }
        }
    }
    panic!("No Winner Found!!!")
}

fn do_bingo(boards: &mut Vec<Board>, nums: &[i32]) -> i32 {
    for num in nums {
        for board in boards.iter_mut() {
            board.mark(*num);
            if board.is_winner() {
                board.calc_score(*num);
                return board.get_score();
            }
        }
    }
    panic!("No Winner Found!!!")
}

fn read_nums(filename: &'static str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let unwraped_line = line.unwrap();
        let nums = unwraped_line
            .split(',')
            .map(|num| num.parse::<i32>().unwrap());
        for num in nums {
            result.push(num);
        }
    }
    result
}

fn read_boards(filename: &'static str) -> Vec<board::Board> {
    let mut result: Vec<board::Board> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut cur_nums: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let unwraped_line = line.unwrap();
        if unwraped_line.is_empty() {
            result.push(board::Board::new(&cur_nums));
            cur_nums.clear();
        } else {
            let nums = unwraped_line.trim().split_whitespace();
            for num in nums {
                cur_nums.push(num.parse::<i32>().unwrap());
            }
        }
    }
    result.push(board::Board::new(&cur_nums));

    result
}
