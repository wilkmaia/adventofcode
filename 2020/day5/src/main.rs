extern crate utils;

use std::collections::HashSet;

use utils::parse_input_to_vec;

#[derive(Debug)]
struct BoardingPass {
    row: u8,
    col: u8,
    id: u16,
}

fn boarding_pass_parser(line: &str) -> BoardingPass {
    let (row_data, col_data) = line.split_at(7);

    let (row, _) = row_data.chars().fold((0, 127), |acc, c| -> (u8, u8) {
        match c {
            'F' => (acc.0, (acc.0 + acc.1)/2),
            'B' => ((acc.0 + acc.1)/2 + 1, acc.1),
            _ => (0, 0),
        }
    });

    let (col, _) = col_data.chars().fold((0, 7), |acc, c| -> (u8, u8) {
        match c {
            'L' => (acc.0, (acc.0 + acc.1)/2),
            'R' => ((acc.0 + acc.1)/2 + 1, acc.1),
            _ => (0, 0),
        }
    });

    BoardingPass {
        row,
        col,
        id: 8 * (row as u16) + col as u16,
    }
}

fn problem1(boarding_passes: &Vec<BoardingPass>) {
    let max = boarding_passes.iter().fold(0, |acc, b| -> u16 {
        if b.id > acc {
            return b.id
        } else {
            acc
        }
    });

    println!("Problem 1 -> {}", max);
}

fn problem2(boarding_passes: &Vec<BoardingPass>) {
    let mut seats = HashSet::new();

    let max = boarding_passes.iter().fold(0, |acc, b| {
        seats.insert(b.id);

        if b.id > acc {
            return b.id
        } else {
            acc
        }
    });

    let mut seat: u16 = 0;

    for i in 1..(max - 2) {
        if seats.contains(&(i - 1)) && seats.contains(&(i + 1)) && !seats.contains(&i) {
            seat = i;
            break;
        }
    }

    println!("Problem 2 -> {}", seat);
}

fn main() {
    let boarding_passes = parse_input_to_vec::<BoardingPass>("input", "\n", boarding_pass_parser);

    problem1(&boarding_passes);
    problem2(&boarding_passes);
}
