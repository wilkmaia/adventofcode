extern crate utils;

use std::collections::HashMap;

use utils::parse_input;

type Memory = HashMap<String, usize>;

const MASK_LEN: usize = 36;

fn v1_update_memory(memory: &mut Memory, line: &str, mask: &String) {
    let addr: String = line.chars()
        .skip(4)
        .take_while(|&c| c != ']')
        .collect();

    let mut val: usize = line.split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse()
        .unwrap();

    let mut idx = 0;
    let mut res = 0;
    while idx < MASK_LEN {
        let mut bit = val % 2;
        match mask.as_str().chars().nth(MASK_LEN - 1 - idx).unwrap() {
            '0' => bit = 0,
            '1' => bit = 1,
            _ => (),
        }

        if bit == 1 {
            res += 2usize.pow(idx as u32);
        }
        val /= 2;
        idx += 1;
    }

    memory.insert(addr, res);
}

fn get_permutations(data: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::new();

    if data.len() == 1 {
        res.push(0);
        res.push(2usize.pow(data[0] as u32));
    }

    for (idx, _) in data.iter().enumerate() {
        let tmp = get_permutations(
            data.iter()
                .skip(idx + 1)
                .map(|&x| x)
                .collect(),
        );

        for &el in tmp.iter() {
            res.push(el);
            res.push(el + 2usize.pow(data[0] as u32));
        }
    }

    res.sort();
    res.dedup();
    res
}

fn v2_update_memory(memory: &mut Memory, line: &str, mask: &String) {
    let addr: String = line.chars()
        .skip(4)
        .take_while(|&c| c != ']')
        .collect();

    let val: usize = line.split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse()
        .unwrap();

    let mut addr_n = addr.parse::<usize>().unwrap();

    let mut idx = 0;
    let mut res = 0;
    let mut floating = Vec::new();
    let mut offset = 0;
    while idx < MASK_LEN {
        let mut bit = addr_n % 2;
        match mask.as_str().chars().nth(MASK_LEN - 1 - idx).unwrap() {
            '1' => bit = 1,
            'X' => {
                floating.push(idx);
                if bit == 1 {
                    offset += 2usize.pow(idx as u32);
                }
            },
            _ => (),
        }

        if bit == 1 {
            res += 2usize.pow(idx as u32);
        }
        addr_n /= 2;
        idx += 1;
    }

    let permutations: Vec<usize> = get_permutations(floating);

    for item in permutations.iter() {
        memory.insert((res + item - offset).to_string(), val);
    }
}

fn memory_parser(
    contents: String,
    update_memory: fn(&mut Memory, &str, &String),
) -> Memory
{
    let lines = contents.split('\n');
    let mut memory = HashMap::new();

    let mut mask: String = String::new(); 

    for line in lines {
        match line.chars().nth(1).unwrap() {
            'a' => mask = line.chars().skip(7).take(MASK_LEN).collect(),
            'e' => update_memory(&mut memory, line, &mask),
            _ => (),
        }
    }

    memory
}

fn v1_memory_parser(contents: String) -> Memory {
    memory_parser(contents, v1_update_memory)
}

fn v2_memory_parser(contents: String) -> Memory {
    memory_parser(contents, v2_update_memory)
}

fn problem1(memory: &Memory) {
    let mut sum = 0;
    for (_, &v) in memory.iter() {
        sum += v;
    }

    println!("Problem 1 -> {}", sum);
}

fn problem2(memory: &Memory) {
    let mut sum = 0;
    for (_, &v) in memory.iter() {
        sum += v;
    }

    println!("Problem 2 -> {}", sum);
}

fn main() {
    let memory = parse_input("input", v1_memory_parser);
    problem1(&memory);

    let memory = parse_input("input", v2_memory_parser);
    problem2(&memory);
}
