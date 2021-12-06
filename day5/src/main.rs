use std::{
    fs::File,
    io::{BufRead, BufReader}
};
use regex::Regex;
mod line;
use line::Line;

fn main() {
    let data = read_data("data.txt");
    println!("Part1 {}", part1(&data));
    println!("Part2 {}", part2(&data));
}

fn part2(lines: &[Line]) -> i32{
    let mut result = 0;
    let mut grid:[[i32;1000];1000] = [[0;1000];1000];
    for line in lines{
        plot_line(&mut grid, line);
    }
    for i in 0..1000{
        for j in 0..1000{
            if grid[i][j] > 1{
                result += 1;
            }
        }
    }
    result
}

fn part1(lines: &[Line]) -> i32{
    let mut result = 0;
    let mut grid:[[i32;1000];1000] = [[0;1000];1000];
    for line in lines{
        if line.x1 == line.x2 || line.y1 == line.y2{
            plot_line(&mut grid, line);
        }
    }
    for i in 0..1000{
        for j in 0..1000{
            if grid[i][j] > 1{
                result += 1;
            }
        }
    }
    result
}


fn plot_line(grid: &mut [[i32;1000];1000], line : &Line){
    if line.x1 == line.x2{
        let y1 = if line.y1 < line.y2 {line.y1} else {line.y2};
        let y2 = if line.y1 > line.y2 {line.y1} else {line.y2};
        for y in y1..=y2{
            grid[y as usize][line.x1 as usize] += 1;
        }
    }else if line.y1 == line.y2{
        let x1 = if line.x1 < line.x2 {line.x1} else {line.x2};
        let x2 = if line.x1 > line.x2 {line.x1} else {line.x2};
        for x in x1 ..= x2{
            grid[line.y1 as usize][x as usize] += 1;
        }
    }else{
        let dx:i32 = if line.x1 > line.x2 {-1} else {1};
        let dy:i32 = if line.y1 > line.y2 {-1} else {1};
        let mut x = line.x1;
        let mut y = line.y1;

        loop{
            if x == line.x2 || y == line.y2{
                break;
            }
            grid[y as usize][x as usize] += 1;
            x += dx;
            y += dy;
        }
    }
}


fn read_data(filename: &str) -> Vec<Line>{
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut result = Vec::new();

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for line in reader.lines(){
        let unwrapped = line.unwrap();
        for cap in re.captures_iter(unwrapped.as_str()){
            result.push(Line::new(
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap()
            ))
        }
    }
    result
}