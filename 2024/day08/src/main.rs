use std::collections::HashSet;
use std::collections::HashMap;

use utils::parse_input;

#[derive(Debug)]
struct DataMap {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

const EPSILON: f64 = 0.00001;

fn parse_input_entries(input: String) -> DataMap {
    let mut height = 0;
    let mut width = 0;
    let mut antennas = HashMap::<char, Vec<(usize, usize)>>::new();

    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            width = line.len();
            height += 1;

            let parts: Vec<char> = line.chars().collect();
            parts.iter().enumerate().for_each(|(j, &c)| {
                if c == '.' {
                    return;
                }

                if !antennas.contains_key(&c) {
                    antennas.insert(c, vec![(i, j)]);
                } else {
                    antennas.get_mut(&c).unwrap().push((i, j));
                }
            });
        });

    DataMap {
        width,
        height,
        antennas
    }
}

fn is_within_bounds(x: isize, y: isize, data_map: &DataMap) -> bool {
    x >= 0 && x < data_map.width as isize && y >= 0 && y < data_map.height as isize
}

fn get_unique_antinodes_positions_for_part_1(data_map: &DataMap) -> HashSet<(usize, usize)> {
    let mut unique_antinode_locations = HashSet::<(usize, usize)>::new();

    for frequency in data_map.antennas.keys() {
        let positions = data_map.antennas.get(frequency).unwrap();

        for i in 0..positions.len() {
            let pos1 = positions[i];

            for j in (i + 1)..positions.len() {
                let pos2 = positions[j];
                let distance = ((pos1.0 as isize - pos2.0 as isize), (pos1.1 as isize - pos2.1 as isize));

                let antinode_1 = ((pos1.0 as isize + distance.0), (pos1.1 as isize + distance.1));
                let antinode_2 = ((pos2.0 as isize - distance.0), (pos2.1 as isize - distance.1));

                if is_within_bounds(antinode_1.0, antinode_1.1, data_map) {
                    unique_antinode_locations.insert((antinode_1.0 as usize, antinode_1.1 as usize));
                }

                if is_within_bounds(antinode_2.0, antinode_2.1, data_map) {
                    unique_antinode_locations.insert((antinode_2.0 as usize, antinode_2.1 as usize));
                }
            }
        }
    }

    unique_antinode_locations
}

fn get_unique_antinodes_positions_for_part_2(data_map: &DataMap) -> HashSet<(usize, usize)> {
    let mut unique_antinode_locations = HashSet::<(usize, usize)>::new();

    for frequency in data_map.antennas.keys() {
        let positions = data_map.antennas.get(frequency).unwrap();

        for i in 0..positions.len() {
            let pos1 = positions[i];

            for j in (i + 1)..positions.len() {
                let pos2 = positions[j];

                // line "y = mx + n"
                let m = (pos2.1 as f64 - pos1.1 as f64) / (pos2.0 as f64 - pos1.0 as f64);
                let n = (pos1.1 as f64) - m * (pos1.0 as f64);

                // Add points in the line going one way
                let mut x = pos1.0 as isize;
                let mut y = pos1.1 as isize;
                'add_points: loop {
                    if !is_within_bounds(x, y, data_map) {
                        break;
                    }

                    unique_antinode_locations.insert((x as usize, y as usize));

                    'find_next_grid_point_in_line: loop {
                        x += 1;
                        if x >= data_map.width as isize {
                            break 'add_points;
                        }

                        let tmp_y = m.mul_add(x as f64, n);
                        if (tmp_y - tmp_y.round()).abs() < EPSILON {
                            y = tmp_y.round() as isize;
                            break 'find_next_grid_point_in_line;
                        }
                    }
                }

                // Add points in the line going the other way
                x = pos1.0 as isize;
                y = pos1.1 as isize;
                'add_points: loop {
                    if !is_within_bounds(x as isize, y as isize, data_map) {
                        break;
                    }

                    unique_antinode_locations.insert((x as usize, y as usize));

                    'find_next_grid_point_in_line: loop {
                        if x == 0 {
                            break 'add_points;
                        }
                        x -= 1;

                        let tmp_y = m.mul_add(x as f64, n);
                        if (tmp_y - tmp_y.round()).abs() < EPSILON {
                            y = tmp_y.round() as isize;
                            break 'find_next_grid_point_in_line;
                        }
                    }
                }
            }
        }
    }

    unique_antinode_locations
}

fn problem1(input: &DataMap) {
    let unique_antinode_locations = get_unique_antinodes_positions_for_part_1(input);

    println!("Problem 1 -> {}", unique_antinode_locations.len());
}

fn problem2(input: &DataMap) {
    let unique_antinode_locations = get_unique_antinodes_positions_for_part_2(input);

    println!("Problem 2 -> {}", unique_antinode_locations.len());
}

fn main() {
    let input = parse_input::<DataMap>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
