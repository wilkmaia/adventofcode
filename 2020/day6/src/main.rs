extern crate utils;

use std::collections::HashSet;
use std::collections::HashMap;

use utils::parse_input;
use utils::basic_parser;

fn problem1(group_answers: &Vec<String>) {
    let count = group_answers.iter().fold(0, |acc, ga| -> usize {
        let mut tmp: Vec<char> = ga
            .chars()
            .filter(|x| *x != '\n')
            .collect();
        let unique_answers: HashSet<_> = tmp
            .drain(..)
            .collect();

        acc + unique_answers.len()
    });

    println!("Problem 1 -> {}", count);
}

fn problem2(group_answers: &Vec<String>) {
    let count = group_answers.iter().fold(0, |acc, ga| -> usize {
        let mut answers = HashMap::new();
        let group_count = ga.split('\n').count();

        ga.chars().for_each(|c| {
            match c {
                'a'..='z' => {
                    if answers.contains_key(&c) {
                        answers.insert(c, answers.get(&c).unwrap() + 1);
                    } else {
                        answers.insert(c, 1);
                    }
                },
                _ => (),
            };
        });

        acc + answers.iter()
            .filter(|(_, v)| **v == group_count)
            .collect::<HashMap<&char, &usize>>()
            .len()
    });

    println!("Problem 2 -> {}", count);
}

fn main() {
    let group_answers = parse_input::<String>("input", "\n\n", basic_parser::<String>);

    problem1(&group_answers);
    problem2(&group_answers);
}
