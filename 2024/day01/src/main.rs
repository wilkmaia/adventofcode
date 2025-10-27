use utils::parse_input;

fn parse_input_entries(input: String) -> Vec<(i32, i32)> {
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    for entry in input.split('\n') {
        let pair_str = entry.split("   ").collect::<Vec<&str>>();
        let pair: (i32, i32) = (
            pair_str[0].parse().unwrap(),
            pair_str[1].parse().unwrap(),
        );
        pairs.push(pair);
    }

    pairs
}

fn get_sorted_pairs(pairs: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut left_values = pairs.iter().map(|p| p.0).collect::<Vec<i32>>();
    let mut right_values = pairs.iter().map(|p| p.1).collect::<Vec<i32>>();

    left_values.sort();
    right_values.sort();

    left_values
        .iter()
        .zip(right_values.iter())
        .map(|(&l, &r)| (l, r))
        .collect::<Vec<(i32, i32)>>()
}

fn problem1(pairs: &Vec<(i32, i32)>) {
    let sorted_pairs = get_sorted_pairs(pairs);
    let distance_sum = sorted_pairs
        .iter()
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("Problem 1 -> {}", distance_sum);
}

fn problem2(pairs: &Vec<(i32, i32)>) {
    let similarity_score = pairs
        .iter()
        .map(|(l, _)| l * pairs.iter().filter(|(_, r)| r == l).count() as i32)
        .sum::<i32>();

    println!("Problem 2 -> {}", similarity_score);
}

fn main() {
    let input = parse_input::<Vec<(i32, i32)>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
