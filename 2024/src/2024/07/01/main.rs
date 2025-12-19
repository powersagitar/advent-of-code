use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
}

type Equation = (usize, Vec<usize>);

fn read_input() -> Vec<Equation> {
    let file = File::open("input.txt").unwrap();
    // let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let colon_idx = line.find(':').unwrap();

            let result = line[..colon_idx].parse().unwrap();
            let equation = line[colon_idx + 2..]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            (result, equation)
        })
        .collect()
}

fn read_operators_from_bits(operators: u32) -> Vec<Operator> {
    (0..(operators.count_zeros() + operators.count_ones()))
        .map(|n| match operators >> n & 1 {
            0 => Operator::Add,
            1 => Operator::Multiply,
            _ => unreachable!(),
        })
        .collect()
}

fn find_operators(equation: &Equation) -> Result<Vec<Operator>, ()> {
    let expected_result = equation.0;
    let operands = &equation.1;
    // use bits to represent the operators, 0 for add, 1 for multiply
    let operators = 2_u32.pow(operands.len() as u32 - 1);

    for operators in 0..operators {
        let operators = read_operators_from_bits(operators);
        let mut operator = operators.iter();
        let mut result = operands[0];

        for window in operands.windows(2) {
            let next_operand = window[1];
            let next_operator = operator.next().unwrap();

            match next_operator {
                Operator::Add => result += next_operand,
                Operator::Multiply => result *= next_operand,
            }
        }

        if result == expected_result {
            return Ok(operators);
        }
    }

    Err(())
}

fn solve(inputs: &[Equation]) -> usize {
    inputs
        .into_iter()
        .filter(|equation| find_operators(equation).is_ok())
        .map(|(result, _)| result)
        .sum()
}

fn main() {
    let input = read_input();
    // println!("{:?}", input);

    let solution = solve(&input);
    println!("solution: {}", solution);
}
