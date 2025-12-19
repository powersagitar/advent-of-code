use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    // let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

const TARGET: &str = "MAS";

fn check_x(input: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // boundary check is done in solve
    // if row + TARGET.len() > input.len() || col + TARGET.len() > input[row].len() {
    //     return false;
    // }

    input[row + 1][col + 1] == 'A'
        && ((input[row][col] == 'M' && input[row + 2][col + 2] == 'S')
            || (input[row][col] == 'S' && input[row + 2][col + 2] == 'M'))
        && ((input[row][col + 2] == 'M' && input[row + 2][col] == 'S')
            || (input[row][col + 2] == 'S' && input[row + 2][col] == 'M'))
}

fn solve(input: &Vec<Vec<char>>) -> usize {
    (0..input.len() - TARGET.len() + 1)
        .map(|row| {
            (0..input[row].len() - TARGET.len() + 1)
                .filter(|&col| check_x(input, row, col))
                .count()
        })
        .sum()
}

fn main() {
    let input = read_input();
    let solution = solve(&input);
    println!("{}", solution);
}
