extern crate utils;

use std::collections::HashMap;

use utils::parse_input;

enum ParsingStep {
    Rules,
    SelfTicket,
    Tickets,
}

type Ticket = Vec<i64>;
type Guess = HashMap<String, Vec<usize>>;

#[derive(Debug)]
struct Notes {
    rules: HashMap<String, Vec<(i64, i64)>>,
    self_ticket: Ticket,
    tickets: Vec<Ticket>,
}

fn parse_rule(notes: &mut Notes, line: &str) -> ParsingStep {
    if line == "" {
        return ParsingStep::SelfTicket;
    }

    let rule_type: String = String::from(
        line.split(':')
            .collect::<Vec<&str>>()[0],
    );
    let range1: Ticket = line.split(": ")
        .collect::<Vec<&str>>()[1]
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .split('-')
        .map(|a| a.parse::<i64>().unwrap())
        .collect();
    let range2: Ticket = line.split("or ")
        .collect::<Vec<&str>>()[1]
        .split('-')
        .map(|a| a.parse::<i64>().unwrap())
        .collect();

    let ranges = vec![(range1[0], range1[1]), (range2[0], range2[1])];
    notes.rules.insert(rule_type, ranges);

    ParsingStep::Rules
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(',')
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

fn parse_self_ticket(notes: &mut Notes, line: &str) -> ParsingStep {
    if line.chars().nth(0).unwrap() == 'y' {
        return ParsingStep::SelfTicket;
    }

    notes.self_ticket = parse_ticket(line);

    ParsingStep::Tickets
}

fn parse_tickets(notes: &mut Notes, line: &str) -> ParsingStep {
    if line == "" || line.chars().nth(0).unwrap() == 'n' {
        return ParsingStep::Tickets;
    }

    notes.tickets.push(parse_ticket(line));

    ParsingStep::Tickets
}

fn notes_parser(contents: String) -> Notes {
    let mut notes = Notes {
        rules: HashMap::new(),
        self_ticket: vec![],
        tickets: vec![],
    };

    let mut step = ParsingStep::Rules;

    for line in contents.split('\n') {
        match step {
            ParsingStep::Rules => step = parse_rule(&mut notes, line),
            ParsingStep::SelfTicket => step = parse_self_ticket(&mut notes, line),
            ParsingStep::Tickets => step = parse_tickets(&mut notes, line),
        }
    }

    notes
}

fn get_invalid_entry(notes: &Notes, ticket: &Ticket) -> Option<i64> {
    for &n in ticket.iter() {
        let mut valid = false;

        for v in notes.rules.values() {
            if valid {
                break;
            }

            for &(min, max) in v.iter() {
                if n >= min && n <= max {
                    valid = true;
                    break;
                }
            }
        }

        if !valid {
            return Some(n);
        }
    }

    None
}

fn solve_columns(notes: &Notes, tickets: Vec<&Ticket>) -> Guess {
    let mut guesses: Guess = HashMap::new();
    notes.rules.keys().for_each(|k| {
        guesses.insert(String::from(k), (0..notes.self_ticket.len()).collect());
    });

    loop {
        tickets.iter()
            .for_each(|t| {
                t.iter().enumerate().for_each(|(idx, &n)| {
                    notes.rules.iter().for_each(|(k, v)| {
                        if !(n >= v[0].0 && n <= v[0].1) &&
                            !(n >= v[1].0 && n <=v[1].1)
                        {
                            let g = guesses.get_mut(k).unwrap();
                            match g.iter().position(|&x| x == idx) {
                                Some(index) => g.remove(index),
                                None => 0,
                            };

                            let elem = g[0];
                            if g.len() == 1 {
                                let _guesses = HashMap::from(guesses.clone());
                                for (key, val) in _guesses {
                                    if key == *k {
                                        continue;
                                    }

                                    match val.iter().position(|&x| x == elem) {
                                        Some(index) => {
                                            let _x = guesses.get_mut(&key).unwrap();
                                            _x.remove(index);
                                        },
                                        None => (),
                                    };
                                }
                            }
                        }
                    });
                });
            });

        let done = guesses.iter().fold(true, |acc, (_, v)| acc && v.len() == 1);
        if done {
            break;
        }
    }

    guesses
}

fn problem1(notes: &Notes) {
    let mut invalid_numbers = Vec::new();

    for ticket in notes.tickets.iter() {
        match get_invalid_entry(notes, ticket) {
            Some(n) => invalid_numbers.push(n),
            None => (),
        }
    }

    println!("Problem 1 -> {}", invalid_numbers.iter().fold(0, |acc, &n| acc + n));
}

fn problem2(notes: &Notes) {
    let valid_tickets: Vec<&Ticket> = notes.tickets
        .iter()
        .filter(|t| get_invalid_entry(notes, t) == None)
        .collect();

    let res = solve_columns(notes, valid_tickets).iter()
        .fold(1, |acc, (k, v)| -> i64 {
            if k.contains("departure") {
                return acc * notes.self_ticket[v[0]];
            }

            acc
        });

    println!("Problem 2 -> {}", res);
}

fn main() {
    let notes = parse_input("input", notes_parser);

    problem1(&notes);
    problem2(&notes);
}
