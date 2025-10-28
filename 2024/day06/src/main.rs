use std::collections::HashSet;

use utils::parse_input;

fn parse_input_entries(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_start_position(input: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '^' || *ch == '>' || *ch == 'v' || *ch == 'V' || *ch == '<' {
                return (i, j);
            }
        }
    }

    panic!("Start position not found");
}

fn get_next_position(
    current_i: i32,
    current_j: i32,
    direction: char,
) -> (i32, i32) {
    match direction {
        '^' => (current_i - 1, current_j),
        'v' | 'V' => (current_i + 1, current_j),
        '<' => (current_i, current_j - 1),
        '>' => (current_i, current_j + 1),
        _ => panic!("Invalid direction"),
    }
}

fn rotate_direction(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'V',
        'v' | 'V' => '<',
        '<' => '^',
        _ => panic!("Invalid direction"),
    }
}

fn get_all_guard_positions(input: &Vec<Vec<char>>) -> (HashSet<(usize, usize)>, bool) {
    let mut positions = HashSet::<(usize, usize)>::new();
    let mut positions_directions_set = HashSet::<(usize, usize, char)>::new();

    let (mut cur_i, mut cur_j) = get_start_position(input);
    let mut direction = input[cur_i][cur_j];
    let mut is_in_loop = false;

    loop {
        positions.insert((cur_i, cur_j));

        let (next_i, next_j) = get_next_position(cur_i as i32, cur_j as i32, direction);
        if next_i >= input.len() as i32 || next_j >= input[0].len() as i32 || next_i < 0 || next_j < 0 {
            break;
        }

        let is_new_entry = positions_directions_set.insert((cur_i, cur_j, direction));
        if !is_new_entry {
            is_in_loop = true;
            break;
        }

        if input[next_i as usize][next_j as usize] == '#' {
            direction = rotate_direction(direction);
            continue;
        }

        cur_i = next_i as usize;
        cur_j = next_j as usize;
    }

    (positions, is_in_loop)
}

fn get_possible_obstruction_positions(input: &Vec<Vec<char>>, positions: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut obstruction_positions = HashSet::<(usize, usize)>::new();

    let start_position = get_start_position(input);
    for (i, j) in positions {
        if *i == start_position.0 && *j == start_position.1 {
            continue;
        }

        let new_input = {
            let mut tmp = input.clone();
            tmp[*i][*j] = '#';
            tmp
        };

        let (_, is_in_loop) = get_all_guard_positions(&new_input);
        if is_in_loop {
            obstruction_positions.insert((*i, *j));
        }
    }

    obstruction_positions
}

fn problem1(input: &Vec<Vec<char>>) {
    let (positions, _) = get_all_guard_positions(input);

    println!("Problem 1 -> {}", positions.len());
}

fn problem2(input: &Vec<Vec<char>>) {
    let (positions, _) = get_all_guard_positions(input);
    let possible_obstruction_positions = get_possible_obstruction_positions(input, &positions);

    println!("Problem 2 -> {}", possible_obstruction_positions.len());
}

fn main() {
    let input = parse_input::<Vec<Vec<char>>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
