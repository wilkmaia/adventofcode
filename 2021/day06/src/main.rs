extern crate utils;

use utils::parse_input_to_vec;

const TIME_TO_REPRODUCE: i32 = 7;
const DAYS_P1: i32 = 80;
const DAYS_P2: i32 = 2;

#[derive(Debug)]
struct Fish {
    days_remaining: i32,
    initial_time_to_reproduce: i32,
}

impl Fish {
    fn get_number_of_children(&self) -> usize {
        if self.days_remaining < self.initial_time_to_reproduce {
            return 0;
        }

        let _n: f64 = ((self.days_remaining - self.initial_time_to_reproduce) as f64)
            / (TIME_TO_REPRODUCE as f64);

        let count = _n.ceil() as usize;
        let mut children: Vec<Fish> = vec![];
        for i in 0..count {
            children.push(Fish {
                days_remaining: self.days_remaining
                    - self.initial_time_to_reproduce
                    - i as i32 * TIME_TO_REPRODUCE,
                initial_time_to_reproduce: TIME_TO_REPRODUCE + 2,
            });
        }

        count
            + children
                .iter()
                .map(|f| f.get_number_of_children())
                .sum::<usize>()
    }
}

fn parser(element: &str) -> Fish {
    let initial_time_to_reproduce = element.trim().parse::<i32>().unwrap();

    Fish {
        days_remaining: DAYS_P1,
        initial_time_to_reproduce,
    }
}

fn problem1(pop: &Vec<Fish>) {
    let total_pop_count = pop.len()
        + pop
            .iter()
            .map(|f| f.get_number_of_children())
            .sum::<usize>();

    println!("Problem 1 -> {}", total_pop_count);
}

// Problem 1's approach isn't optimal for large ranges
// This approach relies on two pieces of data:
//   - The time a lanternfish takes to reproduce is 7 days
//   - Exceptionally, a newly bred lanternfish takes 9 days to reproduce
//
// We'll cycle over a 9 elements vector (9 possible different states for
// each lanternfish) and increment each slot with the number of fish bred
// after `day` days passed.
//
// Each vector index means the time to reproduction of each lanternfish
// and the value stored at that index is the total number of lanternfish
// in that status.
//
// In the time-representing loop below, `t` stands for "today", the current
// day, and `idx` is used to infer how many new lanternfish will be bred and
// in which status they will be, 7 days (breeding cycle) "from today"
fn problem2(initial_population: &Vec<Fish>) {
    let mut pop: Vec<i64> = vec![0; 9];

    initial_population
        .iter()
        .for_each(|f| pop[f.initial_time_to_reproduce as usize] += 1);

    let cycle_period = TIME_TO_REPRODUCE + 2;
    for day in 0..DAYS_P2 {
        let t = day % cycle_period;
        let idx = (t + 7) % cycle_period;

        pop[idx as usize] += pop[t as usize];
    }

    println!("{:?}", pop);
    println!("Problem 2 -> {}", pop.iter().sum::<i64>());
}

fn main() {
    let initial_population = parse_input_to_vec::<Fish>("input", ",", parser);

    problem1(&initial_population);
    problem2(&initial_population);
}
