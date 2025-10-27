use std::collections::HashMap;

use utils::parse_input;

struct Input {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

fn parse_input_entries(input: String) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules = HashMap::<i32, Vec<i32>>::new();
    for line in parts[0].lines() {
        let rule_parts: Vec<&str> = line.split("|").collect();
        let key = rule_parts[0].parse::<i32>().unwrap();
        let value = rule_parts[1].parse::<i32>().unwrap();

        if !rules.contains_key(&key) {
            let v = vec![value];
            rules.insert(key, v);
        } else {
            rules.get_mut(&key).unwrap().push(value);
        }
    }

    let updates = parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Input { rules, updates }
}

fn is_update_correct(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    update
        .iter()
        .enumerate()
        .all(|(idx, num)| {
            let num_rules = rules.get(num);
            if num_rules.is_none() {
                return true;
            }

            !num_rules.unwrap().iter().any(|rule| update[0..idx].contains(rule))
        })
}

fn get_updates(input: &Input, is_correct: bool) -> Vec<&Vec<i32>> {
    input.updates
        .iter()
        .filter(|update| is_correct == is_update_correct(update, &input.rules))
        .collect::<Vec<&Vec<i32>>>()
}

fn get_correct_updates(input: &Input) -> Vec<&Vec<i32>> {
    get_updates(input, true)
}

fn get_incorrect_updates(input: &Input) -> Vec<&Vec<i32>> {
    get_updates(input, false)
}

fn adjust_incorrect_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut adjusted_update = update.clone();

    loop {
        for idx in 0..adjusted_update.len() {
            let num = adjusted_update[idx];
            let num_rules = rules.get(&num);
            if num_rules.is_none() {
                continue;
            }

            for rule in num_rules.unwrap() {
                if adjusted_update[0..idx].contains(rule) {
                    let match_idx = adjusted_update
                        .iter()
                        .position(|&x| x == *rule)
                        .unwrap();

                    let tmp = adjusted_update[match_idx];
                    adjusted_update[match_idx] = adjusted_update[idx];
                    adjusted_update[idx] = tmp;
                }
            }
        }

        if is_update_correct(&adjusted_update, rules) {
            break;
        }
    }

    adjusted_update
}

fn problem1(input: &Input) {
    let correct_updates = get_correct_updates(input);
    let middle_elements_sum = correct_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<i32>();

    println!("Problem 1 -> {}", middle_elements_sum);
}

fn problem2(input: &Input) {
    let incorrect_updates = get_incorrect_updates(input);
    let adjusted_updates = incorrect_updates
        .iter()
        .map(|update| adjust_incorrect_update(update, &input.rules))
        .collect::<Vec<Vec<i32>>>();
    let middle_elements_sum = adjusted_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<i32>();

    println!("Problem 2 -> {}", middle_elements_sum);
}

fn main() {
    let input = parse_input::<Input>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
