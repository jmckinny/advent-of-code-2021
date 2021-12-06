use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
mod line;
use line::Line;

fn main() {
    let data = read_data("data.txt");
    println!("Part1 {}", part1(&data));
    println!("Part2 {}", part2(&data));
}

fn part2(lines: &[Line]) -> i32 {
    let mut grid = vec![[0; 1000]; 1000];
    for line in lines {
        plot_line(&mut grid, line);
    }

    grid.concat()
        .iter()
        .fold(0, |acc, x| if *x > 1 { acc + 1 } else { acc })
}

fn part1(lines: &[Line]) -> i32 {
    let mut grid = vec![[0; 1000]; 1000];
    for line in lines {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            plot_line(&mut grid, line);
        }
    }
    grid.concat()
        .iter()
        .fold(0, |acc, x| if *x > 1 { acc + 1 } else { acc })
}

fn plot_line(grid: &mut Vec<[i32; 1000]>, line: &Line) {
    let dx: i32 = (line.x2 - line.x1).signum();
    let dy: i32 = (line.y2 - line.y1).signum();
    let mut x = line.x1;
    let mut y = line.y1;

    while (x, y) != (line.x2 + dx, line.y2 + dy) {
        grid[y as usize][x as usize] += 1;
        x += dx;
        y += dy;
    }
}

fn read_data(filename: &str) -> Vec<Line> {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut result = Vec::new();

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        for cap in re.captures_iter(unwrapped.as_str()) {
            result.push(Line::new(
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            ))
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let data = read_data("test_data.txt");
        assert_eq!(5, part1(&data));
    }
    #[test]
    fn part2_test() {
        let data = read_data("test_data.txt");
        assert_eq!(12, part2(&data));
    }
}
