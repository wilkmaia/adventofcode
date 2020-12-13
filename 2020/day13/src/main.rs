extern crate utils;

use utils::parse_input;

#[derive(Debug)]
struct Notes {
    min_time: usize,
    bus_ids: Vec<(usize, usize)>,
}

fn notes_parser(contents: String) -> Notes {
    let mut lines = contents.split('\n');

    let min_time = lines.next().unwrap().parse::<usize>().unwrap();
    let mut bus_ids = Vec::new();
    for (offset, id) in lines.next().unwrap().split(',').enumerate() {
        if id == "x" {
            continue;
        }

        let pair = (offset, id.parse::<usize>().unwrap());
        bus_ids.push(pair);
    }

    Notes {
        min_time,
        bus_ids,
    }
}

fn problem1(notes: &Notes) {
    let mut res = std::usize::MAX;
    let mut diff = std::usize::MAX;
    for pair in notes.bus_ids.iter() {
        let id = pair.1;

        let new_diff = id - (notes.min_time % id);
        if new_diff < diff {
            diff = new_diff;
            res = id;
        }
    }

    println!("Problem 1 -> {}", res * diff);
}

fn problem2(notes: &Notes) {
    let mut ids = notes.bus_ids.clone();
    ids.sort_by(|(_, x), (_, y)| y.cmp(x));

    let base_offset = ids[0].0;
    let mut base_id = ids[0].1;

    let res = ids.iter().skip(1).fold(0, |acc, &(offset, id)| -> usize {
        let mut t = acc;
        while (t + offset - base_offset) % id != 0 {
            t += base_id;
        }
        base_id *= id;
        t
    });

    println!("Problem 2 -> {}", res - base_offset);
}

fn main() {
    let notes = parse_input("input", notes_parser);

    problem1(&notes);
    problem2(&notes);
}
