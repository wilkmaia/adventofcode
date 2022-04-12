extern crate utils;

use std::collections::HashSet;
use std::collections::HashMap;

use utils::parse_input_to_vec_to_hashmap;

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

fn get_holders_key_values(rules: &HashMap<String, Rule>) -> Vec<(String, String)> {
    let mut tmp = Vec::<(String, String)>::new();

    for (color, rule) in rules.iter() {
        for hold_rule in rule.holds.iter() {
            tmp.push((String::from(&hold_rule.color), String::from(color)));
        }
    }

    tmp
}

fn update_holded_by(rules: &mut HashMap<String, Rule>, tmp: Vec<(String, String)>) {
    for (k, v) in tmp.iter() {
        rules.get_mut(k)
            .unwrap()
            .holded_by
            .push(String::from(v));
    }
}

fn rule_parser(rule: &str) -> (String, Rule) {
    let words = rule.split_whitespace().collect::<Vec<&str>>();
    let color = format!("{} {}", words[0], words[1]);

    if words[4] == "no" {
        let rule = Rule {
            holds: Vec::new(),
            holded_by: Vec::new(),
        };
        return (color, rule);
    }

    let hold_rules = (5..(words.len())).step_by(4)
        .fold(Vec::new(), |mut r, idx| -> Vec<HoldRule> {
            let count = words[idx - 1].parse::<usize>().unwrap();
            let in_color = format!("{} {}", words[idx], words[idx+1]);

            r.push(HoldRule {
                color: in_color,
                count,
            });
            r
        });

    let rule = Rule {
        holds: hold_rules,
        holded_by: Vec::new(),
    };
    (color, rule)
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
    let mut rules = parse_input_to_hashmap::<String, Rule>(
        "input",
        "\n",
        rule_parser,
    );
    let tmp = get_holders_key_values(&mut rules);
    update_holded_by(&mut rules, tmp);

    problem1(&mut rules);
    problem2(&mut rules);
}
