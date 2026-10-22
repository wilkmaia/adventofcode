use utils::parse_input_to_vec;

struct IdRange {
    first: i64,
    last: i64,
}

fn parse_range(range_str: &str) -> IdRange {
    let range_numbers = range_str.split("-").collect::<Vec<&str>>();

    IdRange {
        first: range_numbers[0].trim().parse().unwrap(),
        last: range_numbers[1].trim().parse().unwrap(),
    }
}

fn problem1(input: &Vec<IdRange>) {
    let mut sum = 0;

    for range in input.iter() {
        for n in range.first..(range.last + 1) {
            if is_id_invalid(n) {
                sum += n;
            }
        }
    }

    println!("Problem 1 -> {}", sum);

    fn is_id_invalid(id: i64) -> bool {
        let id_str = format!("{}", id);
        let n_chars = id_str.len();

        if n_chars % 2 == 1 {
            return false;
        }

        return id_str[0..(n_chars / 2)] == id_str[(n_chars / 2)..];
    }
}

fn problem2(input: &Vec<IdRange>) {
    let mut sum = 0;

    for range in input.iter() {
        for n in range.first..(range.last + 1) {
            if is_id_invalid(n) {
                sum += n;
            }
        }
    }

    println!("Problem 2 -> {}", sum);

    fn is_id_invalid(id: i64) -> bool {
        let id_str = format!("{}", id);
        let n_chars = id_str.len();

        for group_size in 1..(n_chars / 2 + 1) {
            if n_chars % group_size != 0 {
                continue;
            }

            let mut all_equal = true;
            let group = &id_str[0..group_size];
            for group_start in (group_size..n_chars).step_by(group_size) {
                if group != &id_str[group_start..(group_start + group_size)] {
                    all_equal = false;
                    break;
                }
            }

            if all_equal {
                return true;
            }
        }

        false
    }
}

fn main() {
    let input = parse_input_to_vec::<IdRange>("input", ",", parse_range);

    problem1(&input);
    problem2(&input);
}
