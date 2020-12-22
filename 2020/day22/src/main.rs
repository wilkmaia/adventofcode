extern crate utils;

use utils::parse_input;

type Card = i32;
type Hand = Vec<Card>;

fn cards_parser(contents: String) -> Vec<Hand> {
    let mut hands = vec![vec![], vec![]];

    let mut p = 0;
    for players_hands in contents.split("\n\n") {
        for line in players_hands.lines().rev() {
            if line.contains("Player") {
                continue;
            }

            hands[p].push(line.parse::<Card>().unwrap());
        }
        p += 1;
    }

    hands
}

fn add_cards_to_deck(hands: &mut Vec<Hand>, winner: usize, c1: Card, c2: Card) {
    if winner == 0 {
        hands[winner].insert(0, c1);
        hands[winner].insert(0, c2);
    } else {
        hands[winner].insert(0, c2);
        hands[winner].insert(0, c1);
    }
}

fn combat_round(hands: &mut Vec<Hand>) {
    let card1 = hands[0].pop().unwrap();
    let card2 = hands[1].pop().unwrap();

    assert_ne!(card1, card2);

    let p;
    if card1 > card2 {
        p = 0;
    } else {
        p = 1;
    }
    add_cards_to_deck(hands, p, card1, card2);
}

fn get_new_game_deck(hand: &Hand, max: i32) -> Hand {
    let mut h = Hand::new();

    let mut i = 0;
    for &c in hand.iter().rev() {
        if i == max {
            break;
        }

        i += 1;
        h.push(c);
    }

    h.iter().rev().map(|&a| a).collect()
}

fn recursive_combat(hands: &mut Vec<Hand>) -> usize {
    let winner;
    let mut cards_played: Vec<(Card, Card)> = vec![];

    loop {
        if hands[0].len() == 0 {
            winner = 1;
            break;
        }

        if hands[1].len() == 0 {
            winner = 0;
            break;
        }

        let calc1 = calculate_points(&hands[0]);
        let calc2 = calculate_points(&hands[1]);

        if cards_played.contains(&(calc1, calc2)) {
            winner = 0;
            break;
        }

        let c1 = hands[0].pop().unwrap();
        let c2 = hands[1].pop().unwrap();

        let round_winner;
        if hands[0].len() >= c1 as usize && hands[1].len() >= c2 as usize {
            let h1 = get_new_game_deck(&hands[0], c1);
            let h2 = get_new_game_deck(&hands[1], c2);
            round_winner = recursive_combat(&mut vec![h1, h2]);
        } else {
            if c1 > c2 {
                round_winner = 0;
            } else {
                round_winner = 1;
            }
        }

        add_cards_to_deck(hands, round_winner, c1, c2);
        cards_played.push((calc1, calc2));
    }

    winner
}

fn calculate_points(hand: &Hand) -> i32 {
    let mut res = 0;

    for n in 1..(hand.len() + 1) {
        res += n as i32 * hand[n - 1];
    }

    res
}

fn problem1(hands: &mut Vec<Hand>) {
    let winner;
    loop {
        if hands[0].len() == 0 {
            winner = &hands[1];
            break;
        }

        if hands[1].len() == 0 {
            winner = &hands[2];
            break;
        }

        combat_round(hands);
    }

    let res = calculate_points(winner);

    println!("Problem 1 -> {}", res);
}

fn problem2(hands: &mut Vec<Hand>) {
    let winner = recursive_combat(hands);
    let res = calculate_points(&hands[winner]);
    println!("Problem 2 -> {}", res);
}

fn main() {
    let mut hands = parse_input("input", cards_parser);

    problem1(&mut hands.clone());
    problem2(&mut hands);
}
