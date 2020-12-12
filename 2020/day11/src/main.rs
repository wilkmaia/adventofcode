extern crate utils;

use utils::parse_input_to_vec;

#[derive(Debug, Clone, Copy)]
enum Seat {
    Free,
    Occupied,
    Floor,
}

impl Seat {
    fn is_free(&self) -> bool {
        match *self {
            Seat::Free => true,
            _ => false,
        }
    }

    fn is_occupied(&self) -> bool {
        match *self {
            Seat::Occupied => true,
            _ => false,
        }
    }

    fn is_floor(&self) -> bool {
        match *self {
            Seat::Floor => true,
            _ => false,
        }
    }

    fn equals(&self, other: &Seat) -> bool {
        match (*self, *other) {
            (Seat::Floor, Seat::Floor) => true,
            (Seat::Free, Seat::Free) => true,
            (Seat::Occupied, Seat::Occupied) => true,
            _ => false,
        }
    }
}

type Row = Vec<Seat>;
type SeatMap = Vec<Row>;

fn seats_parser(line: &str) -> Row {
    let mut seats = Vec::new();

    for seat in line.chars() {
        match seat {
            'L' => seats.push(Seat::Free),
            '#' => seats.push(Seat::Occupied),
            _ => seats.push(Seat::Floor),
        }
    }

    seats
}

fn count_occupied_neighbours(seats: &SeatMap, i: usize, j: usize, vision_range: usize) -> usize {
    let mut found_seat = (
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );

    let mut count = 0;
    let col_len = seats.len();
    let row_len = seats[0].len();

    for n in 1..(vision_range + 1) {
        let mut in_bound = false;

        if i >= n {
            if seats[i - n][j].is_occupied() && !found_seat.0 {
                count += 1;
                found_seat.0 = true;
            } else if seats[i - n][j].is_free() {
                found_seat.0 = true;
            }

            if j >= n && !found_seat.1 {
                if seats[i - n][j - n].is_occupied() {
                    count += 1;
                    found_seat.1 = true;
                } else if seats[i - n][j - n].is_free() {
                    found_seat.1 = true;
                }
            }

            if j + n < row_len && !found_seat.2 {
                if seats[i - n][j + n].is_occupied() {
                    count += 1;
                    found_seat.2 = true;
                } else if seats[i - n][j + n].is_free() {
                    found_seat.2 = true;
                }
            }

            in_bound = true;
        }

        if i + n < col_len {
            if seats[i + n][j].is_occupied() && !found_seat.3 {
                count += 1;
                found_seat.3 = true;
            } else if seats[i + n][j].is_free() {
                found_seat.3 = true;
            }

            if j >= n && !found_seat.4 {
                if seats[i + n][j - n].is_occupied() {
                    count += 1;
                    found_seat.4 = true;
                } else if seats[i + n][j - n].is_free() {
                    found_seat.4 = true;
                }
            }

            if j + n < row_len && !found_seat.5 {
                if seats[i + n][j + n].is_occupied() {
                    count += 1;
                    found_seat.5 = true;
                } else if seats[i + n][j + n].is_free() {
                    found_seat.5 = true;
                }
            }

            in_bound = true;
        }

        if j >= n && !found_seat.6 {
            if seats[i][j - n].is_occupied() {
                count += 1;
                found_seat.6 = true;
            } else if seats[i][j - n].is_free() {
                found_seat.6 = true;
            }

            in_bound = true;
        }

        if j + n < row_len && !found_seat.7 {
            if seats[i][j + n].is_occupied() {
                count += 1;
                found_seat.7 = true;
            } else if seats[i][j + n].is_free() {
                found_seat.7 = true;
            }

            in_bound = true;
        }

        if !in_bound {
            break;
        }
    }

    count
}

fn apply_rule(
    seats: &SeatMap,
    vision_range: usize,
    max_neighbours: usize,
) -> SeatMap
{
    let mut new_seats = Vec::new();

    for (i, row) in seats.iter().enumerate() {
        let mut new_row = Vec::new();

        for (j, &seat) in row.iter().enumerate() {
            if seat.is_floor() {
                new_row.push(Seat::Floor);
                continue;
            }

            let occupied_neighbours = count_occupied_neighbours(seats, i, j, vision_range);
            if seat.is_free() && occupied_neighbours == 0 {
                new_row.push(Seat::Occupied);
            } else if seat.is_occupied() && occupied_neighbours >= max_neighbours {
                new_row.push(Seat::Free);
            } else {
                new_row.push(seat);
            }
        }

        new_seats.push(new_row);
    }

    new_seats
}

fn count_occupied_seats(seats: &SeatMap) -> usize {
    seats.iter().fold(0, |acc, row: &Row| -> usize {
        acc + row.iter()
            .filter(|&&seat| seat.is_occupied())
            .collect::<Vec<&Seat>>()
            .len()
    })
}

fn seatmaps_equal(first: &SeatMap, second: &SeatMap) -> bool {
    let mut equals = true;

    for (i, row) in first.iter().enumerate() {
        for (j, &seat) in row.iter().enumerate() {
            equals = equals && seat.equals(&second[i][j])
        }
    }

    equals
}

fn solve_problem(
    initial_seats: &SeatMap,
    vision_range: usize,
    max_neighbours: usize,
) -> usize
{
    let mut changed = true;
    let mut count = 0;
    let mut new_seats = apply_rule(initial_seats, vision_range, max_neighbours);

    while changed {
        let tmp = new_seats.clone();
        new_seats = apply_rule(&new_seats, vision_range, max_neighbours);
        if seatmaps_equal(&new_seats, &tmp) {
            changed = false;
            count = count_occupied_seats(&new_seats);
        }
    }

    count
}

fn problem1(seats: &SeatMap) {
    println!("Problem 1 -> {}", solve_problem(seats, 1, 4));
}

fn problem2(seats: &SeatMap) {
    let vision_range = std::cmp::max(seats.len(), seats[0].len());
    println!("Problem 2 -> {}", solve_problem(seats, vision_range, 5));
}

fn main() {
    let seats = parse_input_to_vec::<Row>(
        "input",
        "\n",
        seats_parser,
    );

    problem1(&seats);
    problem2(&seats);
}
