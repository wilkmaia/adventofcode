use regex;

use utils::parse_input;

const FIND_MUL_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

fn parse_input_entries(memory: String) -> String {
    memory
}

fn get_mul_blocks(memory: &String) -> Vec<i32> {
    let re = regex::Regex::new(FIND_MUL_REGEX).unwrap();
    let mut result = Vec::new();

    for cap in re.captures_iter(memory) {
        let a: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

        result.push(a * b);
    }

    result
}

fn get_mul_blocks_considering_do_and_dont_instructions(memory: &String) -> Vec<i32> {
    let mut memory_areas_to_consider: Vec<String> = Vec::new();

    let mut area_splits_by_dont = memory.split("don't()");
    memory_areas_to_consider.push(String::from(area_splits_by_dont.next().unwrap()));

    for area in area_splits_by_dont {
        let area_splits_by_do = area.split("do()");
        let mut splits_iter = area_splits_by_do.into_iter();

        // Skip the first area after "don't()" as it's not to be considered
        splits_iter.next();

        for area_to_consider in splits_iter {
            memory_areas_to_consider.push(String::from(area_to_consider));
        }
    }

    let mut result = Vec::new();
    for area in memory_areas_to_consider {
        let mut mul_blocks = get_mul_blocks(&area);
        result.append(&mut mul_blocks);
    }

    result
}

fn problem1(memory: &String) {
    let mul_blocks = get_mul_blocks(memory);
    let mul_blocks_sum = mul_blocks.iter().sum::<i32>();

    println!("Problem 1 -> {}", mul_blocks_sum);
}

fn problem2(memory: &String) {
    let mul_blocks = get_mul_blocks_considering_do_and_dont_instructions(memory);
    let mul_blocks_sum = mul_blocks.iter().sum::<i32>();

    println!("Problem 2 -> {}", mul_blocks_sum);
}

fn main() {
    let input = parse_input::<String>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
