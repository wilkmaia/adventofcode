use utils::parse_input;

struct Equation {
    expected_result: i64,
    operands: Vec<i64>
}

const POSSIBLE_OPERATORS_PART_1: [&str; 2] = ["+", "*"];
const POSSIBLE_OPERATORS_PART_2: [&str; 3] = ["+", "*", "||"];

fn parse_equation(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(": ").collect();
    let expected_result = parts[0].parse::<i64>().unwrap();
    let operands = parts[1]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Equation {
        expected_result,
        operands
    }
}

fn parse_input_entries(input: String) -> Vec<Equation> {
    input
        .lines()
        .map(parse_equation)
        .collect::<Vec<Equation>>()
}

fn is_equation_possibly_true(equation: &Equation, possible_operators: Vec<&str>) -> bool {
    let num_operands = equation.operands.len();
    let num_operator_slots = num_operands - 1;
    let num_operator_combinations = possible_operators.len().pow(num_operator_slots as u32);

    for i in 0..num_operator_combinations {
        let mut index = i;

        let mut result = equation.operands[0];
        for j in 0..num_operator_slots {
            let operator_index = index % possible_operators.len();
            let op = possible_operators[operator_index];
            index /= possible_operators.len();

            match op {
                "+" => result += equation.operands[j + 1],
                "*" => result *= equation.operands[j + 1],
                "||" => result = format!("{}{}", result, equation.operands[j + 1])
                    .parse::<i64>()
                    .unwrap(),
                _ => panic!("Unknown operator"),
            }
        }

        if result == equation.expected_result {
            return true;
        }
    }

    false
}

fn problem1(input: &Vec<Equation>) {
    let possibly_true_equations = input
        .iter()
        .filter(|eq| is_equation_possibly_true(eq, POSSIBLE_OPERATORS_PART_1.to_vec()))
        .collect::<Vec<&Equation>>();
    let sum = possibly_true_equations
        .iter()
        .map(|eq| eq.expected_result)
        .sum::<i64>();

    println!("Problem 1 -> {}", sum);
}

fn problem2(input: &Vec<Equation>) {
    let possibly_true_equations = input
        .iter()
        .filter(|eq| is_equation_possibly_true(eq, POSSIBLE_OPERATORS_PART_2.to_vec()))
        .collect::<Vec<&Equation>>();
    let sum = possibly_true_equations
        .iter()
        .map(|eq| eq.expected_result)
        .sum::<i64>();

    println!("Problem 2 -> {}", sum);
}

fn main() {
    let input = parse_input::<Vec<Equation>>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
