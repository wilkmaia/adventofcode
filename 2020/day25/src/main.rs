extern crate utils;

use utils::parse_input;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Player {
    Door,
    Card,
}

#[derive(Debug)]
struct Handshake {
    pub_key: i64,
    loop_size: usize,
    subject_number: i64,
}

impl Handshake {
    fn new(pub_key: i64) -> Handshake {
        Handshake {
            pub_key,
            loop_size: 0,
            subject_number: 7,
        }
    }
}

const DIVISOR: i64 = 20201227;
type Data = HashMap::<Player, Handshake>;

fn handshake_parser(contents: String) -> Data {
    let mut lines = contents.lines();

    let card_key = lines.next().unwrap().parse::<i64>().unwrap();
    let door_key = lines.next().unwrap().parse::<i64>().unwrap();

    let mut res = HashMap::new();
    res.insert(Player::Door, Handshake::new(door_key));
    res.insert(Player::Card, Handshake::new(card_key));

    res
}

fn set_loop_size(data: &mut Data) {
    data.iter_mut().for_each(|(_, h)| {
        let mut pub_key = 1i64;
        let mut loop_size = 0usize;

        loop {
            pub_key = (pub_key * h.subject_number) % DIVISOR;
            loop_size += 1;
            if pub_key == h.pub_key {
                h.loop_size = loop_size;
                break;
            }
        }
    });
}

fn problem1(data: &mut Data) {
    set_loop_size(data);

    let door = data.get(&Player::Door).unwrap();
    let card = data.get(&Player::Card).unwrap();

    let (loop_size, pub_key) = if door.loop_size < card.loop_size {
        (door.loop_size, card.pub_key)
    } else {
        (card.loop_size, door.pub_key)
    };

    let mut enc_key = 1;
    for _ in 0..loop_size {
        enc_key = (enc_key * pub_key) % DIVISOR;
    }

    println!("Problem 1 -> {}", enc_key);
}

fn main() {
    let mut data = parse_input("input", handshake_parser);

    problem1(&mut data);
}
