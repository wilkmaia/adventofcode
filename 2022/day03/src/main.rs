extern crate utils;
extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;
use utils::parse_input_to_vec;

#[derive(Debug)]
struct Rucksack {
    line: String,
    comp_1: Vec<char>,
    comp_2: Vec<char>,
}

impl Rucksack {
    fn parse(line: &str) -> Self {
        let count = line.len() / 2;

        let c1_items: Vec<char> = line.chars().take(count).collect();
        let c2_items: Vec<char> = line.chars().skip(count).take(count).collect();

        Rucksack {
            line: String::from(line),
            comp_1: c1_items,
            comp_2: c2_items,
        }
    }

    fn get_priority(self: &Self) -> u32 {
        self.comp_1
            .iter()
            .filter(|c| self.comp_2.contains(c))
            .unique()
            .map(|c| get_character_priority(*c))
            .map(Result::unwrap)
            .sum()
    }
}

fn get_character_priority(c: char) -> Result<u32, &'static str> {
    match c {
        'a'..='z' => Ok((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Ok((c as u32) - ('A' as u32) + 27),
        _ => Err("Invalid character"),
    }
}

fn find_team_badge(team: &[Rucksack]) -> char {
    let mut chars_map: HashMap<char, u32> = HashMap::new();

    team
        .iter()
        .for_each(|rucksack| {
            rucksack
                .line
                .chars()
                .unique()
                .for_each(|c| {
                    if chars_map.contains_key(&c) {
                        chars_map.insert(c, chars_map.get(&c).unwrap() + 1);
                    } else {
                        chars_map.insert(c, 1);
                    }
                });
        });

    *chars_map
        .keys()
        .find_map(|k| {
            if chars_map.get(k) == Some(&3) {
                return Some(k);
            }

            None
        })
        .unwrap()
}

fn problem1(input: &Vec<Rucksack>) {
    let result: u32 = input
        .iter()
        .map(Rucksack::get_priority)
        .sum();

    println!("Problem 1 -> {}", result);
}

fn problem2(input: &Vec<Rucksack>) {
    let result: u32 = input
        .chunks(3)
        .map(|team_chunk| find_team_badge(team_chunk))
        .map(|c| get_character_priority(c))
        .map(Result::unwrap)
        .sum();

    println!("Problem 2 -> {}", result);
}

fn main() {
    let input = parse_input_to_vec::<Rucksack>("input", "\n", Rucksack::parse);

    problem1(&input);
    problem2(&input);
}
