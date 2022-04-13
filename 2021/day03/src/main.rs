extern crate utils;

use std::str::from_utf8;

use utils::basic_parser;
use utils::parse_input_to_vec;

fn problem1(data: &Vec<String>) {
    let words = data.len();
    assert!(words > 0);

    let most_common_freq = words / 2;
    let word_size = data[0].len();
    let mut gamma_rate_vec = vec!['0' as u8; word_size];
    let mut epsilon_rate_vec = vec!['0' as u8; word_size];

    for idx in 0..word_size {
        let mut ones = 0;
        let mut zeros = 0;

        for word in data.iter() {
            if word.as_bytes()[idx] == '1' as u8 {
                ones += 1;
            } else {
                zeros += 1;
            }

            if ones > most_common_freq {
                gamma_rate_vec[idx] = '1' as u8;
                epsilon_rate_vec[idx] = '0' as u8;
                break;
            }

            if zeros > most_common_freq {
                gamma_rate_vec[idx] = '0' as u8;
                epsilon_rate_vec[idx] = '1' as u8;
                break;
            }
        }
    }

    let gamma_rate = i64::from_str_radix(from_utf8(&gamma_rate_vec).unwrap(), 2).unwrap();
    let epsilon_rate = i64::from_str_radix(from_utf8(&epsilon_rate_vec).unwrap(), 2).unwrap();
    println!(
        "Problem 1 -> {} x {} = {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}

fn get_rating(data: &Vec<String>, word_size: usize, most_common: bool) -> i64 {
    let mut _tmp: Vec<String> = data.clone();
    for idx in 0..word_size {
        let mut ones = 0;
        let mut zeros = 0;

        let most_common_freq = _tmp.len() as f32 / 2.0;

        for word in _tmp.iter() {
            if word.as_bytes()[idx] == '1' as u8 {
                ones += 1;
            } else {
                zeros += 1;
            }

            if ones as f32 > most_common_freq
                || zeros as f32 > most_common_freq
                || (ones == zeros && ones as f32 == most_common_freq)
            {
                _tmp = _tmp
                    .into_iter()
                    .filter(|x| {
                        x.as_bytes()[idx]
                            == match most_common {
                                true => (if ones >= zeros { '1' } else { '0' }) as u8,
                                false => (if ones < zeros { '1' } else { '0' }) as u8,
                            }
                    })
                    .collect();
                break;
            }
        }

        if _tmp.len() == 1 {
            break;
        }
    }

    i64::from_str_radix(from_utf8(_tmp[0].as_bytes()).unwrap(), 2).unwrap()
}

fn problem2(data: &Vec<String>) {
    let words = data.len();
    assert!(words > 0);

    let word_size = data[0].len();

    let oxygen_generator_rating = get_rating(data, word_size, true);
    let co2_scrubber_rating = get_rating(data, word_size, false);

    println!(
        "Problem 2 -> {} x {} = {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn main() {
    let data = parse_input_to_vec::<String>("input", "\n", basic_parser::<String>);

    problem1(&data);
    problem2(&data);
}
