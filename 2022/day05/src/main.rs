extern crate utils;

use utils::parse_input;

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

#[derive(Debug, Clone)]
struct Movement {
    qty: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Cargo {
    stacks: Vec<Stack>,
    movements: Vec<Movement>,
}

impl Stack {
    fn remove(self: &mut Self) -> char {
        self.crates.pop().unwrap()
    }

    fn add(self: &mut Self, c: char) {
        self.crates.push(c)
    }

    fn remove_n(self: &mut Self, qty: usize) -> Vec<char> {
        self.crates.split_off(self.crates.len() - qty)
    }

    fn add_n(self: &mut Self, mut cs: Vec<char>) {
        self.crates.append(&mut cs)
    }
}

impl Cargo {
    fn get_top_crates(self: &Self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.crates.iter().last().unwrap())
            .collect::<String>()
    }

    fn execute_orders_cratemover_9000(self: &mut Self) {
        self.movements
            .iter()
            .for_each(|movement| {
                let from = movement.from;
                let to = movement.to;

                (0..movement.qty).for_each(|_| {
                    let c = self.stacks[from].remove();
                    self.stacks[to].add(c);
                });
            })
    }

    fn execute_orders_cratemover_9001(self: &mut Self) {
        self.movements
            .iter()
            .for_each(|movement| {
                let cs = self.stacks[movement.from].remove_n(movement.qty);
                self.stacks[movement.to].add_n(cs);
            })
    }
}

fn parse_stacks<'a>(crates: impl Iterator<Item = &'a str>) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = Vec::new();

    crates
        .take_while(|line| !line.starts_with(" 1"))
        .for_each(|line| {
            for idx in (0..line.len()).step_by(4) {
                let n = idx / 4;
                if stacks.len() < n + 1 {
                    stacks.push(Stack { crates: Vec::new() });
                }

                let c = line.chars().nth(idx + 1).unwrap();
                if c.is_whitespace() {
                    continue;
                }

                stacks[n].crates.insert(0, c);
            }
        });

    stacks
}

fn parse_movements<'a>(orders: impl Iterator<Item = &'a str>) -> Vec<Movement> {
    let mut movements: Vec<Movement> = Vec::new();

    orders
        .for_each(|order| {
            let words: Vec<&str> = order.split(' ').collect();

            movements.push(Movement {
                qty: words[1].parse().unwrap(),
                from: words[3].parse::<usize>().unwrap() - 1,
                to: words[5].parse::<usize>().unwrap() - 1,
            });
        });

    movements
}

fn parse_cargo(contents: String) -> Cargo {
    let initial_crates_arrangement = contents.lines().take_while(|l| !l.is_empty());
    let movement_orders = contents.lines().skip(initial_crates_arrangement.clone().count() + 1);

    Cargo {
        stacks: parse_stacks(initial_crates_arrangement),
        movements: parse_movements(movement_orders),
    }
}

fn problem1(mut cargo: Cargo) {
    cargo.execute_orders_cratemover_9000();

    println!("Problem 1 -> {}", cargo.get_top_crates());
}

fn problem2(mut cargo: Cargo) {
    cargo.execute_orders_cratemover_9001();

    println!("Problem 2 -> {}", cargo.get_top_crates());
}

fn main() {
    let input: Cargo = parse_input("input", parse_cargo);

    problem1(input.clone());
    problem2(input);
}
