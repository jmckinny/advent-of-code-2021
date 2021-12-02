use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let data = read_data("data.txt");
    println!("{:?}", run_commands(&data));
    println!("{:?}", run_commands_2(&data));
}

fn run_commands(data: &[Command]) -> i32 {
    let mut pos = (0, 0);
    for command in data {
        match command.dir {
            Direction::Forward => {
                pos.0 += command.val;
            }
            Direction::Up => pos.1 -= command.val,
            Direction::Down => pos.1 += command.val,
        }
    }
    pos.0 * pos.1
}
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}
fn run_commands_2(data: &[Command]) -> i32 {
    let mut pos = Position { x: 0, y: 0, aim: 0 };
    for command in data {
        match command.dir {
            Direction::Forward => {
                pos.x += command.val;
                pos.y += command.val * pos.aim;
            }
            Direction::Up => pos.aim -= command.val,
            Direction::Down => pos.aim += command.val,
        }
    }
    pos.x * pos.y
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    val: i32,
}
#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

fn read_data(filename: &'static str) -> Vec<Command> {
    let mut result: Vec<Command> = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let unwraped_line = line.unwrap();
        let split_line: Vec<&str> = unwraped_line.split_whitespace().collect();
        let (dir, val) = (split_line[0], split_line[1]);
        let num = val.parse::<i32>().unwrap();
        let direction: Direction;
        match dir {
            "forward" => direction = Direction::Forward,
            "up" => direction = Direction::Up,
            "down" => direction = Direction::Down,
            _ => panic!("Unkown direction!"),
        };
        result.push(Command {
            dir: direction,
            val: num,
        })
    }
    result
}
