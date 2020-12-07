use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct HoldRule {
    color: String,
    count: usize,
}

#[derive(Debug)]
struct Rule {
    holds: Vec<HoldRule>,
    holded_by: Vec<String>,
}

fn update_holded_by(rules: &mut HashMap<String, Rule>, in_color: &String, color: &String) {
    match rules.get_mut(in_color) {
        Some(rule) => { rule.holded_by.push(String::from(color)) },
        None => {
            let rule = Rule {
                holds: Vec::new(),
                holded_by: Vec::from([String::from(color)]),
            };
            rules.insert(String::from(in_color), rule);
        },
    }
}

fn add_rule(rules: &mut HashMap<String, Rule>, rule: &str) {
    let words = rule.split_whitespace().collect::<Vec<&str>>();
    let color = format!("{} {}", words[0], words[1]);

    if words[4] == "no" {
        return;
    }

    let hold_rules = (5..(words.len())).step_by(4)
        .fold(Vec::new(), |mut r, idx| -> Vec<HoldRule> {
            let count = words[idx - 1].parse::<usize>().unwrap();
            let in_color = format!("{} {}", words[idx], words[idx+1]);
            update_holded_by(rules, &in_color, &color);
            r.push(HoldRule {
                color: in_color,
                count,
            });
            r
        });

    match rules.get_mut(&color) {
        Some(rule) => { rule.holds = hold_rules },
        None => {
            let rule = Rule {
                holds: hold_rules,
                holded_by: Vec::new(),
            };
            rules.insert(color, rule);
        },
    }
}

fn initialize_rules(rules: &mut HashMap<String, Rule>) {
    let mut file = File::open("./input").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for rule in contents.split("\n") {
        add_rule(rules, rule);
    }
}

fn set_holders(rules: &mut HashMap<String, Rule>, holders: &mut HashSet<String>, holded_by: &Vec<String>) {
    holded_by.iter()
        .for_each(|c| {
            holders.insert(String::from(c));

            let in_holded_by = rules.get(c)
                .unwrap()
                .holded_by
                .clone();

            set_holders(rules, holders, &in_holded_by);
        });
}

fn problem1(rules: &mut HashMap<String, Rule>) {
    let mut holders = HashSet::new();

    let holded_by = rules.get(&String::from("shiny gold"))
        .unwrap()
        .holded_by
        .clone();

    set_holders(rules, &mut holders, &holded_by);
    println!("Problem 1 -> {}", holders.len());
}

fn count_holded(rules: &mut HashMap<String, Rule>, inner_bags: &Vec<HoldRule>) -> usize {
    inner_bags.iter()
        .fold(1, |acc, b| -> usize {
            let b_inner_bags = rules.get(&String::from(&b.color))
                .unwrap()
                .holds
                .clone();

            acc + b.count * count_holded(rules, &b_inner_bags)
        })
}

fn problem2(rules: &mut HashMap<String, Rule>) {
    let inner_bags = rules.get(&String::from("shiny gold"))
        .unwrap()
        .holds
        .clone();

    let count = count_holded(rules, &inner_bags) - 1;
    println!("Problem 2 -> {}", count);
}

fn main() {
    let mut rules = HashMap::new();
    initialize_rules(&mut rules);

    problem1(&mut rules);
    problem2(&mut rules);
}
