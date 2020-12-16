extern crate utils;

use std::collections::HashMap;

use utils::basic_parser;
use utils::parse_input_to_vec;

#[derive(Debug)]
struct Game {
    turn: i32,
    spoken: i32,
    last_number: i32,
    previous_numbers: HashMap<i32, i32>,
}

fn run_turn(game: &mut Game) {
    let n;
    match game.previous_numbers.get(&game.spoken) {
        Some(&x) => n = game.turn - x,
        None => n = 0,
    };

    game.previous_numbers.insert(game.spoken, game.turn);
    game.last_number = game.spoken;
    game.spoken = n;
    game.turn += 1;
}

fn solve(game: &mut Game, max_turn: i32) {
    while game.turn < max_turn {
        run_turn(game);
    }
}

fn problem1(game: &mut Game) {
    solve(game, 2020);
    println!("Problem 1 -> {}", game.spoken);
}

fn problem2(game: &mut Game) {
    solve(game, 30000000);
    println!("Problem 2 -> {}", game.spoken);
}

fn main() {
    let input = parse_input_to_vec::<i32>("input", ",", basic_parser);

    let mut game = Game {
        turn: input.len() as i32,
        spoken: -1,
        last_number: -1,
        previous_numbers: HashMap::new(),
    };

    for (idx, &n) in input.iter().enumerate() {
        game.previous_numbers.insert(game.spoken, idx as i32);
        game.last_number = game.spoken;
        game.spoken = n;
    }

    problem1(&mut game);
    problem2(&mut game);
}
