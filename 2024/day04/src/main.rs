use utils::parse_input;

fn parse_input_entries(input: String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let blocks: Vec<&str> = input.split_whitespace().collect();
    for block in blocks {
        let entries: Vec<char> = block.chars().collect();
        result.push(entries);
    }

    result
}

fn find_xmas_word(map: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> i32 {
    const WORD_TO_FIND: &str = "XMAS";

    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // down-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1),  // up-right
    ];

    let rows = map.len();
    let cols = map[0].len();
    let word_len = WORD_TO_FIND.len();
    let mut count = 0;

    for (dr, dc) in directions.iter() {
        let mut r = start_row as i32;
        let mut c = start_col as i32;
        let mut match_found = true;

        for k in 0..word_len {
            if r < 0 || r >= rows as i32 || c < 0 || c >= cols as i32 || map[r as usize][c as usize] != WORD_TO_FIND.chars().nth(k).unwrap() {
                match_found = false;
                break;
            }
            r += dr;
            c += dc;
        }

        if match_found {
            count += 1;
        }
    }

    count
}

fn has_x_mas_in_position(map: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {
    if start_row < 1 || start_row + 1 >= map.len() || start_col < 1 || start_col + 1 >= map[0].len() {
        return false;
    }

    if map[start_row][start_col] != 'A' {
        return false;
    }

    // '\' diagonal
    let has_one_mas = (map[start_row - 1][start_col - 1] == 'M' && map[start_row + 1][start_col + 1] == 'S')
        || (map[start_row + 1][start_col + 1] == 'M' && map[start_row - 1][start_col - 1] == 'S');

    // '/' diagonal
    let has_another_mas = (map[start_row - 1][start_col + 1] == 'M' && map[start_row + 1][start_col - 1] == 'S')
        || (map[start_row + 1][start_col - 1] == 'M' && map[start_row - 1][start_col + 1] == 'S');

    has_one_mas && has_another_mas
}

fn problem1(memory: &Vec<Vec<char>>) {
    let total_count = memory.iter()
        .enumerate()
        .map(|(r, _)| {
            memory[r].iter()
                .enumerate()
                .map(|(c, _)| find_xmas_word(memory, r, c))
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Problem 1 -> {}", total_count);
}

fn problem2(memory: &Vec<Vec<char>>) {
    let total_count = memory.iter()
        .enumerate()
        .map(|(r, _)| {
            memory[r].iter()
                .enumerate()
                .filter(|(c, _)| has_x_mas_in_position(memory, r, *c))
                .count()
        })
        .sum::<usize>();

    println!("Problem 2 -> {}", total_count);
}

fn main() {
    let input = parse_input::<Vec<Vec<char>>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
