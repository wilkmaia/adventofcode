use std::fs::File;
use std::io::prelude::*;

fn initialize_array(arr: &mut Vec<String>) {
    let mut file = File::open("./input").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.split('\n') {
        arr.push(String::from(line));
    }
}

fn solve_for_slope(matrix: &Vec<String>, slope: (usize, usize)) -> i64 {
    let cols = matrix[0].len();

    let mut x = 0usize;
    matrix.iter().step_by(slope.1).fold(0, |acc, line| -> i64 {
        let spot = line.chars().nth(x).unwrap();
        x = (x + slope.0) % cols;

        if spot == '#' {
            return acc + 1
        }

        acc
    })
}

fn problem1(matrix: &Vec<String>) {
    println!("Problem 1 -> {}", solve_for_slope(matrix, (3, 1)));
}

fn problem2(matrix: &Vec<String>) {
    let slopes = vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2));
    let res = slopes.iter().fold(1, |acc, slope| acc * solve_for_slope(matrix, *slope));

    println!("Problem 2 -> {}", res);
}

fn main() {
    let mut matrix = Vec::<String>::new();

    initialize_array(&mut matrix);

    problem1(&matrix);
    problem2(&matrix);
}
