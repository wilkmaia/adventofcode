extern crate utils;

use utils::parse_input_to_vec;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Sum,
    Mul,
    ParOpen,
    ParClose,
}

#[derive(Debug, Clone)]
struct Expression {
    operands: Vec<i64>,
    operators: Vec<Operator>,
}

fn expression_parser(line: &str) -> Expression {
    let mut expr = Expression {
        operands: vec![],
        operators: vec![],
    };

    for item in line.split_whitespace() {
        match item {
            "+" => expr.operators.push(Operator::Sum),
            "*" => expr.operators.push(Operator::Mul),
            _ => {
                let count_par_open = item.chars().filter(|&c| c == '(')
                    .collect::<String>()
                    .len();
                let count_par_close = item.chars().filter(|&c| c == ')')
                    .collect::<String>()
                    .len();

                let raw_num = item.chars().filter(|c| c.is_digit(10))
                    .collect::<String>();

                for _ in 0..count_par_open {
                    expr.operators.push(Operator::ParOpen);
                }

                expr.operands.push(raw_num.parse::<i64>().unwrap());

                for _ in 0..count_par_close {
                    expr.operators.push(Operator::ParClose);
                }
            },
        };
    }

    expr
}

fn solve(expr: &mut Expression, sum_takes_precedence: bool) -> i64 {
    if sum_takes_precedence {
        solve_sums(expr);
    }

    expr.operands = expr.operands.iter().rev().map(|&a| a).collect();

    let mut res = expr.operands.pop().unwrap();

    let mut op_idx = 0;
    while op_idx < expr.operators.len() {
        let op = expr.operators[op_idx];
        match op {
            Operator::Sum => res += expr.operands.pop().unwrap(),
            Operator::Mul => res *= expr.operands.pop().unwrap(),
            _ => (),
        };

        op_idx += 1;
    }

    res
}

fn solve_parenthesis(expr: &mut Expression, sum_takes_precedence: bool) {
    while expr.operators.contains(&Operator::ParOpen) {
        let mut po_idx = expr.operators.iter().position(|&o| o == Operator::ParOpen).unwrap();

        loop {
            let n_po_idx = match expr.operators.iter().skip(po_idx + 1).position(|&o| o == Operator::ParOpen) {
                Some(n) => {
                    po_idx + n + 1
                },
                None => po_idx,
            };

            if n_po_idx == po_idx {
                break
            }

            po_idx = n_po_idx;
        }

        let p_count = expr.operators.iter().take(po_idx).fold(0, |acc, o| match o {
            Operator::ParOpen => acc + 1,
            Operator::ParClose => acc + 1,
            _ => acc,
        });
        let pc_idx = po_idx + expr.operators.iter().skip(po_idx).position(|&o| o == Operator::ParClose).unwrap();
        let min_n_idx = po_idx - p_count;
        let max_n_idx = pc_idx - p_count;

        let operands = Vec::from(&expr.operands[min_n_idx..max_n_idx]);
        let operators = Vec::from(&expr.operators[(po_idx + 1)..pc_idx]);
        let e = &mut Expression {
            operands,
            operators,
        };
        let n = solve(e, sum_takes_precedence);

        expr.operators = expr.operators.iter()
            .enumerate()
            .filter(|(idx, _)| *idx < po_idx || *idx > pc_idx)
            .map(|(_, &a)| a)
            .collect();

        expr.operands = expr.operands.iter()
            .enumerate()
            .filter(|(idx, _)| *idx < min_n_idx || *idx >= max_n_idx)
            .map(|(_, &a)| a)
            .collect();

        expr.operands.insert(min_n_idx, n);
    }
}

fn solve_sums(expr: &mut Expression) {
    while expr.operators.contains(&Operator::Sum) {
        let s_idx = expr.operators.iter().position(|&o| o == Operator::Sum).unwrap();

        let min_n_idx = s_idx;
        let max_n_idx = s_idx + 2;

        let operands = Vec::from(&expr.operands[min_n_idx..max_n_idx]);
        let operators = vec![Operator::Sum];
        let e = &mut Expression {
            operands,
            operators,
        };
        let n = solve(e, false);

        expr.operators = expr.operators.iter()
            .enumerate()
            .filter(|(idx, _)| *idx != s_idx)
            .map(|(_, &a)| a)
            .collect();

        expr.operands = expr.operands.iter()
            .enumerate()
            .filter(|(idx, _)| *idx < min_n_idx || *idx >= max_n_idx)
            .map(|(_, &a)| a)
            .collect();

        expr.operands.insert(min_n_idx, n);
    }
}

fn problem1(expressions: &Vec<Expression>) {
    let mut s = 0;
    for mut expr in expressions.clone() {
        solve_parenthesis(&mut expr, false);
        s += solve(&mut expr, false);
    }
    println!("Problem 1 -> {}", s);
}

fn problem2(expressions: &Vec<Expression>) {
    let mut s = 0;
    for mut expr in expressions.clone() {
        solve_parenthesis(&mut expr, true);
        s += solve(&mut expr, true);
    }
    println!("Problem 2 -> {}", s);
}

fn main() {
    let expressions = parse_input_to_vec::<Expression>(
        "input",
        "\n",
        expression_parser,
    );

    problem1(&expressions);
    problem2(&expressions);
}
