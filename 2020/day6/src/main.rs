use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn initialize_array(arr: &mut Vec<String>) {
    let mut file = File::open("./input").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for group_answers in contents.split("\n\n") {
        arr.push(String::from(group_answers));
    }
}

fn problem1(group_answers: &Vec<String>) {
    let count = group_answers.iter().fold(0, |acc, ga| -> usize {
        let mut unique_answers = HashSet::new();

        ga.chars().for_each(|c| {
            match c {
                'a'..='z' => { unique_answers.insert(c); },
                _ => (),
            };
        });

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
    let mut group_answers = Vec::<String>::new();
    initialize_array(&mut group_answers);

    problem1(&group_answers);
    problem2(&group_answers);
}
