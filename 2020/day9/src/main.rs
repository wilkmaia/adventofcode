extern crate utils;

use utils::parse_input_to_vec;
use utils::basic_parser;

const PREAMBLE_SIZE: usize = 25;

fn is_sum_possible(pool: &Vec<&i64>, sum: i64) -> bool {
    let mut sorted_pool = pool.clone();
    sorted_pool.sort_unstable();

    let valid_pool = sorted_pool.iter()
        .filter(|&&&n| n < sum)
        .collect::<Vec<&&i64>>();

    let mut l_iter = valid_pool.iter();
    let mut r_iter = valid_pool.iter().rev();

    // println!("{} - {:?}", sum, pool);
    let mut l_item = ***l_iter.next().unwrap();
    let mut r_item = ***r_iter.next().unwrap();

    loop {
        // println!("{} + {}", l_item, r_item);

        if l_item == r_item {
            return false
        }

        if l_item + r_item == sum {
            return true
        } else if l_item + r_item > sum {
            r_item = ***r_iter.next().unwrap();
        } else {
            l_item = ***l_iter.next().unwrap();
        }
    }
}

fn get_invalid_element_idx(numbers: &Vec<i64>) -> usize {
    let mut pool = numbers.iter()
        .take(PREAMBLE_SIZE)
        .collect::<Vec<&i64>>();

    for idx in (PREAMBLE_SIZE)..(numbers.len()) {
        if !is_sum_possible(&pool, numbers[idx]) {
            return idx;
        }

        pool.remove(0);
        pool.push(&numbers[idx]);
    };

    return 0;
}

fn get_contiguous_list(numbers: &Vec<i64>, sum: i64) -> Vec<i64> {
    let mut l_idx = 0;

    while l_idx < numbers.len() - 1 {
        let mut r_idx = l_idx + 1;
        let mut aux_sum = numbers[l_idx];

        while r_idx < numbers.len() {
            aux_sum += numbers[r_idx];

            if aux_sum == sum {
                return Vec::from(&numbers.as_slice()[l_idx..(r_idx + 1)])
            } else if aux_sum < sum {
                r_idx += 1;
                continue;
            } else {
                break;
            }
        }

        l_idx += 1;
    };

    Vec::new()
}

fn problem1(numbers: &Vec<i64>) {
    println!("Problem 1 -> {}", numbers[get_invalid_element_idx(numbers)])
}

fn problem2(numbers: &Vec<i64>) {
    let idx = get_invalid_element_idx(numbers);

    let mut list = get_contiguous_list(numbers, numbers[idx]);
    list.sort();

    let sum = list[0] + list[list.len() - 1];
    println!("Problem 2 -> {}", sum);
}

fn main() {
    let numbers = parse_input_to_vec::<i64>("input", "\n", basic_parser::<i64>);

    problem1(&numbers);
    problem2(&numbers);
}
