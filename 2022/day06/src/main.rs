extern crate utils;
extern crate itertools;

use utils::parse_input;
use itertools::Itertools;

fn get_index_for_first_n_distinct_characters(buffer: &String, n: usize) -> Option<usize> {
    let mut sub: Vec<char> = (&buffer[0..n]).chars().collect();
    if sub.iter().unique().count() == n {
        return Some(0 as usize);
    }

    let mut idx = 1;
    for c in buffer.chars().skip(n) {
        sub[(idx - 1) % n] = c;

        if sub.iter().unique().count() == n {
            return Some(idx);
        }

        idx += 1;
    }

    None
}

fn get_start_of_packet_index(buffer: &String) -> Option<usize> {
    get_index_for_first_n_distinct_characters(buffer, 4)
}

fn get_start_of_message_index(buffer: &String) -> Option<usize> {
    get_index_for_first_n_distinct_characters(buffer, 14)
}

fn problem1(input: &String) {
    let start_of_packet_index = get_start_of_packet_index(input).unwrap();
    println!("Problem 1 -> {}", start_of_packet_index + 4 as usize);
}

fn problem2(input: &String) {
    let start_of_message_index = get_start_of_message_index(input).unwrap();
    println!("Problem 2 -> {}", start_of_message_index + 14 as usize);
}

fn main() {
    let input: String = parse_input("input", |s| s);

    problem1(&input);
    problem2(&input);
}
