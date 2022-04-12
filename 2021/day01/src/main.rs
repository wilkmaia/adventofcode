extern crate utils;

use utils::basic_parser;
use utils::parse_input_to_vec;

fn problem1(data: &Vec<i32>) {
    assert!(data.len() >= 1);

    let mut previous_read: i32 = data[0];
    let mut count: i32 = 0;
    for read in data.iter() {
        if *read > previous_read {
            count += 1;
        }
        previous_read = *read;
    }

    println!("Problem 1 -> {}", count);
}

fn problem2(data: &Vec<i32>) {
    let l = data.len();
    assert!(l >= 3);

    let mut previous_read: i32 = data[0] + data[1] + data[2];
    let mut count: i32 = 0;

    for n in 1..(l - 2) {
        let read = data[n] + data[n + 1] + data[n + 2];
        if read > previous_read {
            count += 1;
        }
        previous_read = read;
    }

    println!("Problem 2 -> {}", count);
}

fn main() {
    let data = parse_input_to_vec::<i32>("input", "\n", basic_parser::<i32>);

    problem1(&data);
    problem2(&data);
}
