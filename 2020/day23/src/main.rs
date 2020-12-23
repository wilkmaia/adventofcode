extern crate utils;

use utils::parse_input;

#[derive(Debug, Clone)]
struct Game {
    cups: Vec<usize>,
    cur: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            cups: vec![],
            cur: 0,
        }
    }

    fn run_round(&mut self, max: usize) {
        let mut picked = vec![0; 3];
        picked[0] = self.cups[self.cur];
        picked[1] = self.cups[self.cups[self.cur]];
        picked[2] = self.cups[self.cups[self.cups[self.cur]]];
        self.cups[self.cur] = self.cups[picked[2]];

        let mut dest = self.cur;
        loop {
            dest = if dest > 1 { dest - 1 } else { max };
            if !picked.contains(&dest) {
                break;
            }
        }

        let tmp = self.cups[dest];
        self.cups[dest] = picked[0];
        self.cups[picked[2]] = tmp;

        self.cur = self.cups[self.cur];
    }

    fn get_result_p1(&self) -> String {
        let mut c = self.cups[1];
        let mut res = String::new();

        while c != 1 {
            res += c.to_string().as_str();
            c = self.cups[c];
        }

        res
    }

    fn get_result_p2(&self) -> usize {
        self.cups[1] * self.cups[self.cups[1]]
    }
}

fn game_parser(contents: String) -> Game {
    let mut g = Game::new();

    g.cups = Vec::with_capacity(10);
    for _ in 0..10 {
        g.cups.push(0);
    }

    let mut last_cup: usize = 0;
    let mut first_cup: usize = 0;
    for c in contents.chars() {
        let n = c.to_digit(10).unwrap() as usize;
        match last_cup {
            0 => first_cup = n,
            _c => g.cups[_c] = n,
        }
        last_cup = n;
    }

    g.cur = first_cup;
    g.cups[last_cup] = first_cup;

    g
}

fn problem1(game: &Game) {
    let mut _g = game.clone();
    for _ in 0..100 {
        _g.run_round(9);
    }

    println!("Problem 1 -> {}", _g.get_result_p1());
}

fn problem2(game: &Game) {
    let mut _g = game.clone();
    let mut tmp = _g.cups;
    _g.cups = Vec::with_capacity(1000001); // Avoid unnecessary reallocations later on
    _g.cups.append(&mut tmp);

    let idx = _g.cups.iter().position(|&n| n == _g.cur).unwrap();
    let tmp = _g.cups[idx];
    _g.cups[idx] = 10;
    for i in 10..1000000 {
        _g.cups.push(i+1);
    }
    _g.cups.push(tmp);

    for _ in 0..10000000 {
        _g.run_round(1000000);
    }

    println!("Problem 2 -> {}", _g.get_result_p2());
}

fn main() {
    let game = parse_input("input", game_parser);

    problem1(&game);
    problem2(&game);
}
