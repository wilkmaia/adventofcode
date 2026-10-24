use std::cmp::max;
use utils::parse_input_to_vec;

#[derive(Debug)]
struct BatteryBank {
    joltages: Vec<u64>,
}

fn parse_battery_bank(joltages_str: &str) -> BatteryBank {
    BatteryBank {
        joltages: joltages_str
            .chars()
            .map(|j| j.to_digit(10).unwrap() as u64)
            .collect(),
    }
}

fn problem1(input: &Vec<BatteryBank>) {
    let mut sum = 0;

    for battery_bank in input {
        let n_joltages = battery_bank.joltages.len();
        let mut max_j: u64 = 0;
        let mut second_max_j: u64 = 0;
        for (j, idx) in battery_bank.joltages.iter().zip(0..n_joltages) {
            if *j > max_j && idx < n_joltages - 1 {
                max_j = *j;
                second_max_j = 0;
                continue;
            }

            if *j > second_max_j {
                second_max_j = *j;
            }

            if max_j == 9 && second_max_j == 9 {
                break;
            }
        }

        sum += 10 * max_j + second_max_j;
    }

    println!("Problem 1 -> {}", sum);
}

fn problem2(input: &Vec<BatteryBank>) {
    let mut sum = 0;

    const N: usize = 12;

    for battery_bank in input {
        let n_joltages = battery_bank.joltages.len();
        let mut max_js: Vec<u64> = vec![0; N];
        for (j, idx) in battery_bank.joltages.iter().zip(0..n_joltages) {
            let start = max(0, idx as isize + N as isize - n_joltages as isize) as usize;
            for i in start..N {
                if *j > max_js[i] && N - i <= n_joltages - idx {
                    max_js[i] = *j;
                    clear_max_js(&mut max_js, i + 1);
                    break;
                }
            }

            if max_js.iter().all(|j| *j == 9) {
                break;
            }
        }

        sum += max_js.iter().zip(0..N).fold(0, |acc, (j, idx)| {
            acc + *j * 10u64.pow(N as u32 - idx as u32 - 1)
        });
    }

    println!("Problem 2 -> {}", sum);

    fn clear_max_js(max_js: &mut Vec<u64>, start_idx: usize) {
        let n = max_js.len();
        if start_idx >= n {
            return;
        }

        for i in start_idx..n {
            max_js[i] = 0;
        }
    }
}

fn main() {
    let input = parse_input_to_vec::<BatteryBank>("input", "\n", parse_battery_bank);

    problem1(&input);
    problem2(&input);
}
