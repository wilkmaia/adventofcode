use utils::parse_input;

#[derive(Debug)]
struct Rotation {
    left: bool,
    count: i32,
}

fn parse_input_entries(input: String) -> Vec<Rotation> {
    let mut rotations: Vec<Rotation> = Vec::new();

    for entry in input.lines() {
        let rotation = Rotation {
            left: entry.chars().nth(0).unwrap() == 'L',
            count: entry[1..].parse().unwrap(),
        };
        rotations.push(rotation);
    }

    rotations
}

fn problem1(rotations: &Vec<Rotation>) {
    let mut dial: i32 = 50;
    let mut count = 0;

    for rotation in rotations.iter() {
        dial += (match rotation.left {
            true => -rotation.count,
            false => rotation.count,
        }) % 100;
        if dial < 0 {
            dial += 100;
        } else if dial >= 100 {
            dial -= 100;
        }

        if dial == 0 {
            count += 1;
        }
    }

    println!("Problem 1 -> {}", count);
}

fn problem2(rotations: &Vec<Rotation>) {
    let mut dial: i32 = 50;
    let mut count = 0;

    for rotation in rotations.iter() {
        let dial_started_at_zero = dial == 0;

        if rotation.left {
            dial -= rotation.count;

            if dial == 0 {
                count += 1;
            } else if dial < 0 {
                count += 1 + (-dial) / 100;
                if dial_started_at_zero {
                    count -= 1;
                }
                dial = ((dial % 100) + 100) % 100;
            }

            continue;
        }

        dial += rotation.count;
        count += dial / 100;
        dial %= 100;
    }

    println!("Problem 2 -> {}", count);
}

fn main() {
    let input = parse_input::<Vec<Rotation>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
