extern crate utils;

use std::collections::HashMap;

use utils::basic_parser;
use utils::parse_input_to_vec;

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct Game {
    input: Vec<u8>,
    boards: Vec<Board>,
}

fn parse_game_data(raw_data: &Vec<String>) -> Game {
    let n_entries = raw_data.len();
    assert!(n_entries > 1);
    assert!((n_entries - 1) % 5 == 0);

    let n_boards = (n_entries - 1) / 5;

    let input = raw_data[0]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut boards: Vec<Board> = Vec::with_capacity(n_boards);
    for board_idx in 0..n_boards {
        let mut rows: Vec<Vec<u8>> = Vec::with_capacity(5);

        for row_idx in 0..5 {
            let idx = board_idx * 5 + row_idx + 1;
            rows.push(
                raw_data[idx]
                    .replace("  ", " ")
                    .split(" ")
                    .filter(|&x| x != "")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
        }

        boards.push(Board { rows });
    }

    Game { input, boards }
}

fn did_board_win(board: &Board, picked_numbers: &HashMap<u8, bool>) -> bool {
    let mut cols_match = vec![true, true, true, true, true];
    for row in board.rows.iter() {
        let mut row_match = true;

        for (col_idx, number) in row.iter().enumerate() {
            if !picked_numbers.contains_key(number) {
                row_match = false;
                cols_match[col_idx] = false;
            }
        }

        if row_match {
            return true;
        }
    }

    cols_match.contains(&true)
}

fn get_sum_of_unpicked_numbers(board: &Board, picked_numbers: &HashMap<u8, bool>) -> u32 {
    let mut sum: u32 = 0;

    for row in board.rows.iter() {
        for number in row.iter() {
            if !picked_numbers.contains_key(number) {
                sum += *number as u32;
            }
        }
    }

    sum
}

fn problem1(game: &Game) {
    let mut picked_numbers: HashMap<u8, bool> = HashMap::new();
    let mut winner_board: Option<&Board> = None;
    let mut last_number = &0;

    'outer_loop: for number in game.input.iter() {
        picked_numbers.insert(*number, true);

        for board in game.boards.iter() {
            if did_board_win(board, &picked_numbers) {
                winner_board = Some(board);
                last_number = number;
                break 'outer_loop;
            }
        }
    }

    if winner_board.is_none() {
        println!("Problem 1 -> NO WINNER");
    } else {
        let board = winner_board.unwrap();
        let sum_of_unpicked_numbers = get_sum_of_unpicked_numbers(board, &picked_numbers);
        println!(
            "Problem 1 -> {}",
            sum_of_unpicked_numbers * (*last_number as u32)
        );
    }
}

fn problem2(game: &Game) {
    let mut picked_numbers: HashMap<u8, bool> = HashMap::new();
    let mut last_winner_board: Option<&Board> = None;
    let mut winner_boards_idx: Vec<usize> = vec![];
    let mut last_number = &0;

    'outer_loop: for number in game.input.iter() {
        picked_numbers.insert(*number, true);

        for (board_idx, board) in game.boards.iter().enumerate() {
            if winner_boards_idx.contains(&board_idx) {
                continue;
            }

            if did_board_win(board, &picked_numbers) {
                winner_boards_idx.push(board_idx);
                if winner_boards_idx.len() == game.boards.len() {
                    last_winner_board = Some(board);
                    last_number = number;
                    break 'outer_loop;
                }
            }
        }
    }

    if last_winner_board.is_none() {
        println!("Problem 2 -> NO WINNER");
    } else {
        let board = last_winner_board.unwrap();
        println!("{:?}", board);
        let sum_of_unpicked_numbers = get_sum_of_unpicked_numbers(board, &picked_numbers);
        println!(
            "Problem 2 -> {}",
            sum_of_unpicked_numbers * (*last_number as u32)
        );
    }
}

fn main() {
    let raw_data = parse_input_to_vec::<String>("input", "\n", basic_parser::<String>);
    let game: Game = parse_game_data(&raw_data);

    problem1(&game);
    problem2(&game);
}
