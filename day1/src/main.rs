use std::fs::File;
use std::io::prelude::*;

fn initialize_vector(arr: &mut Vec<i32>) {
    let mut file = File::open("./input1").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.split('\n') {
        arr.push(line.parse::<i32>().unwrap());
    }
}

fn problem2() -> Option<i32> {
    let mut data: Vec<i32> = Vec::new();
    initialize_vector(&mut data);

    data.sort();

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

fn problem1() -> Option<i32> {
    let mut data: Vec<i32> = Vec::new();
    initialize_vector(&mut data);

    data.sort();

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
    println!("Problem 1 -> {}", problem1().unwrap());
    println!("Problem 2 -> {}", problem2().unwrap());
}
