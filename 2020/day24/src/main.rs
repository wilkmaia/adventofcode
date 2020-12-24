extern crate utils;

use utils::basic_parser;
use utils::parse_input_to_vec;
use std::collections::HashMap;

type Maze = HashMap::<(i32, i32), Color>;

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl Color {
    fn flip(&self) -> Color {
        use Color::*;
        match self {
            Black => White,
            White => Black,
        }
    }

    fn is_black(&self) -> bool {
        use Color::*;
        match self {
            Black => true,
            White => false,
        }
    }
}

#[derive(Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
    TMP(char),
}

impl Direction {
    fn from(line: &String) -> Vec<Direction> {
        use Direction::*;
        line.chars().fold(vec![], |mut acc, c| -> Vec<Direction> {
            match c {
                'e' => {
                    if acc.len() >= 1 {
                        match acc[acc.len() - 1] {
                            TMP(_c) => {
                                acc.pop();
                                match _c {
                                    's' => acc.push(SE),
                                    'n' => acc.push(NE),
                                    _ => println!("{}e", _c),
                                };
                            },
                            _ => acc.push(E),
                        };
                    } else {
                        acc.push(E);
                    }
                },
                'w' => {
                    if acc.len() >= 1 {
                        match acc[acc.len() - 1] {
                            TMP(_c) => {
                                acc.pop();
                                match _c {
                                    's' => acc.push(SW),
                                    'n' => acc.push(NW),
                                    _ => println!("{}w", _c),
                                };
                            },
                            _ => acc.push(W),
                        };
                    } else {
                        acc.push(W);
                    }
                },
                _ => acc.push(TMP(c)),
            };

            acc
        })
    }

    fn move_pos(&self, pos: &mut (i32, i32)) {
        use Direction::*;
        match self {
            E => pos.0 += 2,
            W => pos.0 -= 2,
            NW => { pos.0 -= 1; pos.1 += 1 },
            SW => { pos.0 -= 1; pos.1 -= 1 },
            NE => { pos.0 += 1; pos.1 += 1 },
            SE => { pos.0 += 1; pos.1 -= 1 },
            TMP(_) => println!("Unexpected TMP"),
        }
    }
}

fn flip_tile(dirs: Vec<Direction>) -> (i32, i32) {
    let mut pos = (0, 0);
    for dir in dirs.iter() {
        dir.move_pos(&mut pos);
    }
    pos
}

fn count_adjacent_black_tiles(tiles: &mut Maze, x: i32, y: i32) -> i32 {
    let pos = vec![
        (x - 2, y),
        (x + 2, y),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y - 1),
    ];

    pos.iter().fold(0, |acc, p| -> i32 {
        match tiles.get(p) {
            Some(t) => if t.is_black() { acc + 1 } else { acc },
            None => acc,
        }
    })
}

fn get_new_black_tiles(tiles: &mut Maze) {
    let mut tiles_vec: Vec<(i32, i32)> = tiles.iter().map(|(&p, _)| p).collect();
    tiles_vec.sort();

    let min_x = tiles_vec[0].0;
    let max_x = tiles_vec[tiles_vec.len() - 1].0;

    let mut ys: Vec<i32> = tiles_vec.iter().map(|&(_, y)| y).collect();
    ys.sort();

    let min_y = ys[0];
    let max_y = ys[ys.len() - 1];

    let mut to_flip = vec![];

    for x in min_x - 2..max_x + 3 {
        for y in min_y - 2..max_y + 3 {
            let count = count_adjacent_black_tiles(tiles, x, y);
            match tiles.get(&(x, y)) {
                Some(t) => {
                    if ((count == 0 || count > 2) && t.is_black())
                        || (count == 2 && !t.is_black())
                    {
                        to_flip.push((x, y));
                        // tiles.insert((x, y), Color::White);
                    }
                },
                None => {
                    if count == 2 {
                        to_flip.push((x, y));
                        // tiles.insert((x, y), Color::Black);
                    }
                },
            }
        }
    }

    to_flip.iter().for_each(|p| match tiles.get_mut(p) {
        Some(c) => { *c = c.flip(); },
        None => { tiles.insert(*p, Color::Black); },
    });
}

fn problem1(lines: &Vec<String>, tiles: &mut Maze) {
    for line in lines.iter() {
        let dirs = Direction::from(line);
        let t = flip_tile(dirs);
        match tiles.get_mut(&t) {
            Some(c) => *c = c.flip(),
            None => { tiles.insert(t, Color::Black); },
        };
    }

    let res = tiles.values().filter(|&v| v.is_black()).collect::<Vec<&Color>>().len();
    println!("Problem 1 -> {}", res);
}

fn problem2(tiles: &mut Maze) {
    for _ in 0..100 {
        get_new_black_tiles(tiles);
    }

    let res = tiles.values().filter(|&v| v.is_black()).collect::<Vec<&Color>>().len();
    println!("Problem 2 -> {}", res);
}

fn main() {
    let lines: Vec<String> = parse_input_to_vec("input", "\n", basic_parser);
    let mut tiles = Maze::new();

    problem1(&lines, &mut tiles);
    problem2(&mut tiles);
}
