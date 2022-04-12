extern crate utils;

use regex::Regex;

use utils::parse_input_to_vec;

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: bool,
}

fn passport_parser(passport_entry: &str) -> Passport {
    let mut passport = Passport {
        byr: 0,
        iyr: 0,
        eyr: 0,
        hgt: String::new(),
        hcl: String::new(), 
        ecl: String::new(),
        pid: String::new(),
        cid: false,
    };

    passport_entry.split_ascii_whitespace()
        .for_each(|key_value| {
            let kv = key_value.split(':').collect::<Vec<&str>>();
            match kv[0] {
                "byr" => passport.byr = kv[1].parse().unwrap(),
                "iyr" => passport.iyr = kv[1].parse().unwrap(),
                "eyr" => passport.eyr = kv[1].parse().unwrap(),
                "hgt" => passport.hgt = String::from(kv[1]),
                "hcl" => passport.hcl = String::from(kv[1]),
                "ecl" => passport.ecl = String::from(kv[1]),
                "pid" => passport.pid = String::from(kv[1]),
                "cid" => passport.cid = true,
                _ => (),
            }
        });

    passport
}

// Compare against default values
fn passport_has_needed_fields(p: &Passport) -> bool {
    (p.byr != 0) &&
    (p.iyr != 0) &&
    (p.eyr != 0) &&
    (p.hgt.len() > 0) &&
    (p.hcl.len() > 0) &&
    (p.ecl.len() > 0) &&
    (p.pid.len() > 0)
}

fn passport_is_valid(p: &Passport, hcl_re: &Regex, pid_re: &Regex) -> bool {
    let hgt_n: u8 = match p.hgt.trim_end_matches(char::is_alphabetic).parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    let hgt_t = p.hgt.trim_start_matches(char::is_numeric);

    let valid_ecl = vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth");

    (p.byr >= 1920 && p.byr <= 2002) &&
    (p.iyr >= 2010 && p.iyr <= 2020) &&
    (p.eyr >= 2020 && p.eyr <= 2030) &&
    (
        (hgt_t == "cm" && hgt_n >= 150 && hgt_n <= 193) ||
        (hgt_t == "in" && hgt_n >= 59 && hgt_n <= 76)
    ) &&
    (hcl_re.is_match(&p.hcl)) &&
    (valid_ecl.contains(&p.ecl.as_str())) &&
    (pid_re.is_match(&p.pid))
}

fn problem1(passports: &Vec<Passport>) {
    let res = passports.iter()
        .fold(0, |acc, p| { if passport_has_needed_fields(&p) { return acc + 1 } acc });

    println!("Problem 1 -> {}", res);
}

fn problem2(passports: &Vec<Passport>) {
    let hcl_re: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    let pid_re: Regex = Regex::new(r"^[0-9]{9}$").unwrap();

    let res = passports.iter()
        .fold(0, |acc, p| { if passport_is_valid(&p, &hcl_re, &pid_re) { return acc + 1 } acc });

    println!("Problem 2 -> {}", res);
}

fn main() {
    let passports = parse_input_to_vec::<Passport>("input", "\n\n", passport_parser);

    problem1(&passports);
    problem2(&passports);
}
