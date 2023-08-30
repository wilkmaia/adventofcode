extern crate utils;

use utils::parse_input;

fn parse_input_entries(input: String) -> Vec<i32> {
    let mut calories_per_elf: Vec<i32> = Vec::new();

    let mut current_calories: i32 = 0;
    for entry in input.split('\n') {
        if entry.is_empty() {
            calories_per_elf.push(current_calories);
            current_calories = 0;
            continue;
        }

        let calories: i32 = entry.parse().unwrap();
        current_calories += calories;
    }
    calories_per_elf.push(current_calories);

    calories_per_elf.sort();
    calories_per_elf.reverse();

    calories_per_elf
}

fn problem1(calories: &Vec<i32>) {
    println!("Problem 1 -> {}", calories[0]);
}

fn problem2(calories: &Vec<i32>) {
    println!("Problem 2 -> {}", calories[0] + calories[1] + calories[2]);
}

fn main() {
    let input = parse_input::<Vec<i32>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
