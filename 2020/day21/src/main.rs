extern crate utils;

use utils::parse_input;
use std::collections::HashMap;

type Mapper = HashMap<String, Vec<String>>;
type Ingredients = HashMap<String, i32>;

fn ingredients_parser(contents: String) -> (Ingredients, Mapper) {
    let mut map = Mapper::new();
    let mut set = Ingredients::new();

    for line in contents.lines() {
        let allergens: Vec<String> = line
            .split(" (contains ")
            .collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|w| w.chars().filter(|&c| c != ')').collect())
            .collect();

        let ingredients: Vec<String> = line
            .split(" (contains")
            .collect::<Vec<&str>>()[0]
            .split_whitespace()
            .map(|i| i.to_string())
            .collect();

        for i in ingredients.iter() {
            match set.get_mut(i) {
                Some(n) => *n += 1,
                None => { set.insert(i.clone(), 1); },
            };
        }

        for _a in allergens.iter() {
            let a = String::from(_a);
            match map.get_mut(&a) {
                Some(d) => {
                    let r: Vec<String> = d.iter()
                        .filter(|e| ingredients.contains(e))
                        .map(|e| e.clone())
                        .collect();
                    map.insert(a.clone(), r);
                },
                None => { map.insert(a.clone(), ingredients.clone()); },
            }
        }
    }

    (set, map)
}

fn problem1(map: &mut Mapper, set: &Ingredients) {
    let mut res = 0;

    for (i, c) in set.iter() {
        let mut count = 0;

        for (_, is) in map.iter() {
            if is.contains(i) {
                count += 1;
            }
        }

        if count == 0 {
            res += c;
        }
    }

    println!("Problem 1 -> {}", res);
}

fn problem2(map: &mut Mapper, set: &Ingredients) {
    let mut solved: Mapper;
    loop {
        solved = map
            .clone()
            .iter()
            .filter(|(_, is)| is.len() == 1)
            .map(|(a, is)| (a.clone(), is.clone()))
            .collect();

        if solved.len() == map.len() {
            break;
        }

        for (_a, _is) in solved.iter() {
            let i = _is[0].clone();
            for (a, is) in map.iter_mut() {
                if _a == a {
                    continue;
                }

                let pos = is.iter()
                    .position(|_i| _i.clone() == i);
                match pos {
                    Some(n) => { is.remove(n); },
                    None => (),
                };
            }
        }
    }

    let mut l: Vec<(&String, String)> = solved
        .iter()
        .map(|(a, is)| (a, is[0].clone()))
        .collect();
    l.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    let l: Vec<String> = l.iter().map(|(_, i)| i.clone()).collect();

    println!("Problem 2 -> {}", l.join(","));
}

fn main() {
    let (set, mut map) = parse_input("input", ingredients_parser);
    problem1(&mut map, &set);
    problem2(&mut map, &set);
}
