extern crate utils;
extern crate regex;

use std::collections::HashMap;
use regex::Regex;

use utils::parse_input;

type RawRules = HashMap<i32, String>;
type Rules = HashMap<i32, String>;
type Messages = Vec<String>;

#[derive(Debug)]
struct Data {
    raw_rules: RawRules,
    rules: Rules,
    messages: Messages,
    loopback: bool,
    re: Regex,
}

impl Data {
    fn parse_rule(&mut self, s: &str) -> String {
        s.split_whitespace()
            .fold(String::new(), |acc, n| -> String {
                let r = self.get_rule(n.parse::<i32>().unwrap());
                if acc.len() == 0 {
                    return r;
                }

                acc + &r
            })
    }

    fn get_rule(&mut self, n: i32) -> String {
        match self.rules.get(&n) {
            Some(r) => return r.clone(),
            None => (),
        };

        let r = self.raw_rules.get(&n).unwrap();
        if r.contains('"') {
            return r.chars().nth(1).unwrap().to_string();
        }

        let expr = r.clone();

        let mut r;
        if n == 8 || n == 11 {
            r = self.parse_rule(expr.split('|')
                .collect::<Vec<&str>>()[0]);
            let _42 = self.get_rule(42);
            let _31 = self.get_rule(31);

            match self.loopback {
                false => r = String::from("(") + &r + ")",
                true => match n {
                    8 => r = String::from("(") + &_42 + ")+",
                    11 => {
                        let mut tmp = vec![];
                        for i in 1..10 {
                            let _t = String::from("(") + &_42 + "{" + &i.to_string() + "}?" + &_31 + "{" + &i.to_string() + "}?" + ")";
                            tmp.push(_t);
                        }
                        r = String::from("(") + &tmp.join("|") + ")";
                    },
                    _ => (),
                },
            };
        } else {
            r = expr.split('|')
                .map(|s| self.parse_rule(s))
                .collect::<Vec<String>>()
                .join("|");
            r = String::from("(") + &r + ")";
        }
            
        self.rules.insert(n, r.clone());
        r
    }

    fn compile(&mut self) {
        let r = String::from("^") + &self.get_rule(0) + "$";
        let mut rb = regex::RegexBuilder::new(&r);
        rb.size_limit(10 * (1 << 20));
        self.re = rb.build().unwrap();
    }

    fn matches(&self, s: &String) -> bool {
        self.re.is_match(s)
    }
}

fn raw_rules_parser(line: &str) -> (i32, String) {
    let s: Vec<&str> = line.split(": ").collect();
    let n = s[0].parse::<i32>().unwrap();

    (n, String::from(s[1]))
}

fn parser(contents: String) -> Data {
    let mut parsed_raw_rules = false;
    let mut raw_rules = RawRules::new();
    let mut messages = vec![];
    for line in contents.split('\n') {
        if line == "" {
            parsed_raw_rules = true;
            continue;
        }

        if !parsed_raw_rules {
            let t = raw_rules_parser(line);
            raw_rules.insert(t.0, t.1);
        } else {
            messages.push(String::from(line));
        }
    }

    Data {
        raw_rules,
        rules: Rules::new(),
        messages,
        loopback: false,
        re: Regex::new(r"").unwrap(),
    }
}

fn problem1(data: &mut Data) {
    data.compile();

    let count = data.messages.iter().fold(0, |acc, m|
        match data.matches(m) {
            false => acc,
            true => acc + 1,
        });

    println!("Problem 1 -> {}", count);
}

fn problem2(data: &mut Data) {
    data.loopback = true;
    data.rules.remove(&0);
    data.rules.remove(&8);
    data.rules.remove(&11);

    data.compile();

    let count = data.messages.iter().fold(0, |acc, m|
        match data.matches(m) {
            false => acc,
            true => acc + 1,
        });

    println!("Problem 2 -> {}", count);
}

fn main() {
    let mut data = parse_input("input", parser);

    problem1(&mut data);
    problem2(&mut data);
}
