extern crate utils;

mod helpers;

use utils::basic_parser;
use utils::parse_input_to_vec;
use helpers::*;

fn problem1(input: &Vec<String>) {
    let result: i32 = input
        .iter()
        .map(|line| {
            let round_result: RoundResult = line.parse().unwrap();
            let my_option: PlayableOption = line.split(' ').last().unwrap().parse().unwrap();

            my_option.get_points() + round_result.get_points()
        })
        .sum();

    println!("Problem 1 -> {}", result);
}

fn problem2(input: &Vec<String>) {
    let result: i32 = input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let oponent_option: PlayableOption = split.next().unwrap().parse().unwrap();
            let round_result: RoundResult = split.next().unwrap().parse().unwrap();
            let my_option = PlayableOption::pick_option_to_achieve_result(&oponent_option, &round_result);

            my_option.get_points() + round_result.get_points()
        })
        .sum();

    println!("Problem 2 -> {}", result);
}

fn main() {
    let input = parse_input_to_vec::<String>("input", "\n", basic_parser::<String>);

    problem1(&input);
    problem2(&input);
}
