extern crate utils;

use utils::parse_input;

struct PasswordRule {
    min: u32,
    max: u32,
    letter: char,
}

struct Password {
    rule: PasswordRule,
    password: String,
}

fn password_parser(line: &str) -> Password {
    let min_str = line.split('-').collect::<Vec<&str>>()[0];
    let max_str = line.split('-').collect::<Vec<&str>>()[1].split(' ').collect::<Vec<&str>>()[0];
    let letter = String::from(line.split(':').collect::<Vec<&str>>()[0]).pop().unwrap();
    let password = String::from(line.split(": ").collect::<Vec<&str>>()[1]);

    Password {
        rule: PasswordRule {
            min: min_str.parse::<u32>().unwrap(),
            max: max_str.parse::<u32>().unwrap(),
            letter,
        },
        password,
    }
}

fn problem1(passwords: &Vec<Password>) {
    let mut valid_count = 0u32;

    for password in passwords {
        let mut letter_count = 0u32;

        for c in password.password.as_str().chars() {
            if c == password.rule.letter {
                letter_count += 1;
            }
        }

        if letter_count >= password.rule.min && letter_count <= password.rule.max {
            valid_count += 1;
        }
    }

    println!("Problem 1 - Valid Count = {}", valid_count);
}

fn problem2(passwords: &Vec<Password>) {
    let mut valid_count = 0u32;

    for password in passwords {
        let mut letter_count = 0u32;
        let chars = password.password.as_bytes();

        if chars[(password.rule.min - 1) as usize] as char == password.rule.letter {
            letter_count += 1
        }

        if chars[(password.rule.max - 1) as usize] as char == password.rule.letter {
            letter_count += 1
        }

        if letter_count == 1 {
            valid_count += 1;
        }
    }

    println!("Problem 2 - Valid Count = {}", valid_count);
}

fn main() {
    let passwords = parse_input::<Password>("input", "\n", password_parser);

    problem1(&passwords);
    problem2(&passwords);
}
