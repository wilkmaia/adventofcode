extern crate utils;

use utils::parse_input_to_vec;
use utils::basic_parser;

const TARGET: i32 = 2020;

fn problem1(data: &Vec<i32>) {
    let (mut i, mut j) = (0, data.len() - 1);

    while i != j {
        let n1 = data[i];
        let n2 = data[j];

        if n1 + n2 == TARGET {
            println!("Problem 1 -> {}", n1 * n2);
            return
        } else if n1 + n2 > TARGET {
            j -= 1;
        } else {
            i += 1;
        }
    }

    println!("No solution for problem 1 with target = {}", TARGET);
}

fn problem2(data: &Vec<i32>) {
    match data.iter()
        .enumerate()
        .find(|(idx, &n)| -> bool {
            let (mut i, mut j) = (idx + 1, data.len() - 1);

            while i != j {
                let n1 = data[i];
                let n2 = data[j];

                if n + n1 + n2 == TARGET {
                    println!("Problem 2 -> {}", n * n1 * n2);
                    return true
                } else if n + n1 + n2 > TARGET {
                    j -= 1;
                } else {
                    i += 1;
                }
            }

            return false
        })
    {
        Some(_) => (),
        None => println!("No solution for problem 2 with target = {}", TARGET)
    };
}

fn main() {
    let mut data = parse_input_to_vec::<i32>("input", "\n", basic_parser::<i32>);
    data.sort();

    problem1(&data);
    problem2(&data);
}
