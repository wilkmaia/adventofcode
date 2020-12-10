use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn initialize_vector(arr: &mut Vec<i32>) {
    let mut file = File::open("./input").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.split('\n') {
        arr.push(line.parse::<i32>().unwrap());
    }
}

fn problem1(joltages: &Vec<i32>) {
    let mut differences = HashMap::<i32, i32>::new();
    differences.insert(1, 0);
    differences.insert(2, 0);
    differences.insert(3, 0);

    let mut last_joltage = 0;
    joltages.iter().for_each(|&j| {
        let diff = j - last_joltage;
        differences.insert(diff, differences.get(&diff).unwrap() + 1);

        last_joltage = j;
    });

    differences.insert(3, differences.get(&3).unwrap() + 1);

    let res = differences.get(&1).unwrap() * differences.get(&3).unwrap();
    println!("Problem 1 -> {}", res);

    println!("{:?}", differences);
}

fn problem2(joltages: &Vec<i32>) {
    let mut count = 0;
    let mut exclude = 0;

    let mut list = joltages.clone();
    list.insert(0, 0);

    println!("{:?}", list);

    for i in 1..(list.len() - 1) {
        let curr = list[i];
        let next = list[i + 1];

        if i == 0 {
            if next - curr == 1 {
                count += 1;
            }

            continue;
        }

        let prev = list[i - 1];

        if curr - prev == 1 && next - curr == 1 {
            count += 1;

            if i >= 3 && next - list[i - 3] == 4 {
                exclude += 1;
                println!("{}", next);
            }
        }
    }

    let res = 2i64.pow(count - (3 * exclude)) * 7i64.pow(exclude);
    println!("Problem 2 -> {}, {}, {}", count, exclude, res);
}

fn main() {
    let mut joltages = Vec::new();
    initialize_vector(&mut joltages);
    joltages.sort();

    problem1(&joltages);
    problem2(&joltages);
}
