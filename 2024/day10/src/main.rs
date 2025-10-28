use std::collections::HashSet;

use utils::parse_input;

type TMap = Vec<Vec<i8>>;

const TRAILHEAD: i8 = 0;
const TRAILEND: i8 = 9;
const ELEVATION_STEP: i8 = 1;

fn parse_input_entries(input: String) -> TMap {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect::<Vec<i8>>())
        .collect::<TMap>()
}

fn is_in_bounds(tmap: &TMap, x: isize, y: isize) -> bool {
    x >= 0
    && y >= 0
    && x < tmap.len() as isize
    && tmap[0].len() >= 1
    && y < tmap[0].len() as isize
}

fn walk_topographic_map_score(
    tmap: &TMap,
    x: isize,
    y: isize,
    prev_x: isize,
    prev_y: isize,
    prev_elevation: i8,
    is_start: bool) -> HashSet<(isize, isize)> {
    let mut res = HashSet::<(isize, isize)>::new();

    if !is_in_bounds(tmap, x, y) {
        return res;
    }

    let current_elevation = tmap[x as usize][y as usize];

    if !is_start {
        if current_elevation - prev_elevation != ELEVATION_STEP {
            return res;
        }

        if current_elevation == TRAILEND {
            res.insert((x, y));
            return res;
        }
    }

    let a = if x + 1 == prev_x && y == prev_y { HashSet::<(isize, isize)>::new() } else { walk_topographic_map_score(tmap, x + 1, y, x, y, current_elevation, false) };
    let b = if x - 1 == prev_x && y == prev_y { HashSet::<(isize, isize)>::new() } else { walk_topographic_map_score(tmap, x - 1, y, x, y, current_elevation, false) };
    let c = if x == prev_x && y + 1 == prev_y { HashSet::<(isize, isize)>::new() } else { walk_topographic_map_score(tmap, x, y + 1, x, y, current_elevation, false) };
    let d = if x == prev_x && y - 1 == prev_y { HashSet::<(isize, isize)>::new() } else { walk_topographic_map_score(tmap, x, y - 1, x, y, current_elevation, false) };

    res.extend(a);
    res.extend(b);
    res.extend(c);
    res.extend(d);

    res
}

fn walk_topographic_map_rating(
    tmap: &TMap,
    x: isize,
    y: isize,
    prev_x: isize,
    prev_y: isize,
    prev_elevation: i8,
    is_start: bool) -> u64 {
    if !is_in_bounds(tmap, x, y) {
        return 0;
    }

    let current_elevation = tmap[x as usize][y as usize];

    if !is_start {
        if current_elevation - prev_elevation != ELEVATION_STEP {
            return 0;
        }

        if current_elevation == TRAILEND {
            return 1;
        }
    }

    let a = if x + 1 == prev_x && y == prev_y { 0 } else { walk_topographic_map_rating(tmap, x + 1, y, x, y, current_elevation, false) };
    let b = if x - 1 == prev_x && y == prev_y { 0 } else { walk_topographic_map_rating(tmap, x - 1, y, x, y, current_elevation, false) };
    let c = if x == prev_x && y + 1 == prev_y { 0 } else { walk_topographic_map_rating(tmap, x, y + 1, x, y, current_elevation, false) };
    let d = if x == prev_x && y - 1 == prev_y { 0 } else { walk_topographic_map_rating(tmap, x, y - 1, x, y, current_elevation, false) };

    a + b + c + d
}

fn get_sum_of_scores_for_part_1(tmap: &TMap) -> usize {
    tmap
        .iter()
        .enumerate()
        .map(|(x, line)| line
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == 0)
            .map(|(y, n)| walk_topographic_map_score(tmap, x as isize, y as isize, x as isize, y as isize, *n, true))
            .map(|hs| hs.len())
            .sum::<usize>())
        .sum::<usize>()
}

fn get_sum_of_ratings_for_part_2(tmap: &TMap) -> u64 {
    tmap
        .iter()
        .enumerate()
        .map(|(x, line)| line
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == 0)
            .map(|(y, n)| walk_topographic_map_rating(tmap, x as isize, y as isize, x as isize, y as isize, *n, true))
            .sum::<u64>())
        .sum::<u64>()
}

fn problem1(input: &TMap) {
    let total_score = get_sum_of_scores_for_part_1(input);

    println!("Problem 1 -> {}", total_score);
}

fn problem2(input: &TMap) {
    let total_rating = get_sum_of_ratings_for_part_2(input);

    println!("Problem 2 -> {}", total_rating);
}

fn print_tmap(tmap: &TMap) {
    for line in tmap {
        for d in line {
            print!("{}", d);
        }

        println!();
    }
}

fn main() {
    let input = parse_input::<TMap>("input", parse_input_entries);
    print_tmap(&input);

    problem1(&input);
    problem2(&input);
}
