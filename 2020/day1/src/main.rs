extern crate utils;

use utils::parse_input;
use utils::basic_parser;

fn problem2(data: &Vec<i32>) -> Option<i32> {
    let mut i = 0usize;
    while i < data.len(){
        let n1 = data[i];
        let mut j = i + 1usize;

        while j < data.len() {
            let n2 = data[j];
            if n1 + n2 > 2020 {
                break;
            }

            let mut k = j + 1usize;
            while k < data.len() {
                let n3 = data[k];
                if n3 > 2020 {
                    break;
                }

                if n1 + n2 + n3 == 2020 {
                    return Some(n1 * n2 * n3);
                }

                k += 1;
            }

            j += 1;
        }

        i += 1;
    }

    return None;
}

fn problem1(data: &Vec<i32>) -> Option<i32> {
    let mut i = 0usize;
    while i < data.len(){
        let n1 = data[i];
        let mut j = i + 1usize;

        while j < data.len() {
            let n2 = data[j];
            if n2 > 2020 {
                break;
            }

            if n1 + n2 == 2020 {
                return Some(n1 * n2);
            }

            j += 1;
        }

        i += 1;
    }

    return None;
}

fn main() {
    let mut data = parse_input::<i32>("input", "\n", basic_parser::<i32>);
    data.sort();

    println!("Problem 1 -> {}", problem1(&data).unwrap());
    println!("Problem 2 -> {}", problem2(&data).unwrap());
}
