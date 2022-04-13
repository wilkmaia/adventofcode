extern crate utils;

use std::collections::HashMap;

use utils::basic_parser;
use utils::parse_input_to_vec;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn from_raw_entry(raw_entry: &str) -> Point {
        let xy: Vec<&str> = raw_entry.split(',').collect();
        Point {
            x: xy[0].parse::<i32>().unwrap(),
            y: xy[1].parse::<i32>().unwrap(),
        }
    }
}

impl Line {
    fn from_raw_line(raw_line: &String) -> Line {
        let pairs: Vec<&str> = raw_line.split(" -> ").collect();

        let start = Point::from_raw_entry(pairs[0]);
        let end = Point::from_raw_entry(pairs[1]);

        Line { start, end }
    }
}

fn fill_horizontal_and_vertical_lines(lines: &Vec<Line>, grid: &mut HashMap<(i32, i32), i32>) {
    for line in lines.iter() {
        if line.start.y == line.end.y {
            let __y = line.start.y;
            let (s, e) = if line.start.x > line.end.x {
                (line.end.x, line.start.x + 1)
            } else {
                (line.start.x, line.end.x + 1)
            };

            for x in s..e {
                grid.entry((x, __y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        if line.start.x == line.end.x {
            let __x = line.start.x;
            let (s, e) = if line.start.y > line.end.y {
                (line.end.y, line.start.y + 1)
            } else {
                (line.start.y, line.end.y + 1)
            };

            for y in s..e {
                grid.entry((__x, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
}

fn fill_diagonal_lines(lines: &Vec<Line>, grid: &mut HashMap<(i32, i32), i32>) {
    for line in lines.iter() {
        let diff_y = line.end.y - line.start.y;
        let diff_x = line.end.x - line.start.x;

        if diff_y.abs() != diff_x.abs() {
            continue;
        }

        let step_x = diff_x / diff_x.abs();
        let step_y = diff_y / diff_y.abs();

        let mut x = line.start.x;
        let mut y = line.start.y;
        for _ in 0..(diff_y.abs() + 1) {
            grid.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
            x += step_x;
            y += step_y;
        }
    }
}

fn problem1(lines: &Vec<Line>, grid: &mut HashMap<(i32, i32), i32>) {
    fill_horizontal_and_vertical_lines(lines, grid);

    let total = grid.values().cloned().filter(|&v| v >= 2).count();

    println!("Problem 1 -> {}", total);
}

fn problem2(lines: &Vec<Line>, grid: &mut HashMap<(i32, i32), i32>) {
    fill_diagonal_lines(lines, grid);

    let total = grid.values().cloned().filter(|&v| v >= 2).count();

    println!("Problem 2 -> {}", total);
}

fn main() {
    let lines: Vec<Line> = parse_input_to_vec::<String>("input", "\n", basic_parser::<String>)
        .iter()
        .map(Line::from_raw_line)
        .collect();

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    problem1(&lines, &mut grid);
    problem2(&lines, &mut grid);
}
