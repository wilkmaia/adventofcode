extern crate utils;

use utils::parse_input_to_vec;

struct ElfAssignment {
    first_section: i32,
    last_section: i32,
}

impl ElfAssignment {
    fn fully_contains_another(self: &Self, other: &Self) -> bool {
        self.first_section <= other.first_section && self.last_section >= other.last_section
    }

    fn overlaps_with_another(self: &Self, other: &Self) -> bool {
        (self.first_section <= other.first_section && other.first_section <= self.last_section)
        || (self.first_section <= other.last_section && other.last_section <= self.last_section)
    }
}

struct ElfAssignmentPair {
    left: ElfAssignment,
    right: ElfAssignment,
}

impl ElfAssignmentPair {
    fn does_one_fully_contain_the_other(self: &Self) -> bool {
        self.left.fully_contains_another(&self.right) || self.right.fully_contains_another(&self.left)
    }

    fn does_one_overlap_with_the_other(self: &Self) -> bool {
        self.left.overlaps_with_another(&self.right) || self.right.overlaps_with_another(&self.left)
    }
}

fn parse_input(line: &str) -> ElfAssignmentPair {
    let data = line
        .split(',')
        .map(|assignment| {
            assignment
                .split('-')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    ElfAssignmentPair {
        left: ElfAssignment { first_section: data[0][0], last_section: data[0][1] },
        right: ElfAssignment { first_section: data[1][0], last_section: data[1][1] },
    }
}

fn problem1(input: &Vec<ElfAssignmentPair>) {
    let count = input
        .iter()
        .filter(|pair| pair.does_one_fully_contain_the_other())
        .count();

    println!("Problem 1 -> {}", count);
}

fn problem2(input: &Vec<ElfAssignmentPair>) {
    let count = input
        .iter()
        .filter(|pair| pair.does_one_overlap_with_the_other())
        .count();

    println!("Problem 2 -> {}", count);
}

fn main() {
    let input = parse_input_to_vec("input", "\n", parse_input);

    problem1(&input);
    problem2(&input);
}
