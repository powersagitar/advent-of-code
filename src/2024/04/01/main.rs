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

fn to_vertical(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut vertical = vec![];

    for i in 0..input.len() {
        let mut row = vec![];
        for j in 0..input.len() {
            row.push(input[j][i]);
        }
        vertical.push(row);
    }

    vertical
}

fn to_vertical_backwards(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = input.clone();
    input.reverse();

    to_vertical(&input)
}

fn to_top_left_bottom_right(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut top_left_bottom_right = vec![];

    // left half
    let mut j = 0;
    for i in 0..input.len() - 3 {
        let mut builder = vec![];

        for k in i..input[0].len() {
            builder.push(input[k][k - j]);
        }

        j += 1;

        top_left_bottom_right.push(builder);
    }

    // right half
    let mut j = 0;
    for i in 0..input[0].len() - 4 {
        let mut builder = vec![];

        for k in i..input[0].len() - 1 {
            builder.push(input[k - j][k + 1]);
        }

        j += 1;

        top_left_bottom_right.push(builder);
    }

    top_left_bottom_right
}

fn to_top_right_bottom_left(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let input = input
        .clone()
        .into_iter()
        .map(|mut row| {
            row.reverse();
            row
        })
        .collect();

    to_top_left_bottom_right(&input)
}

fn to_bottom_left_top_right(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = input.clone();
    input.reverse();

    to_top_left_bottom_right(&input)
}

fn to_bottom_right_top_left(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut input = input.clone();
    input.reverse();

    to_top_right_bottom_left(&input)
}

fn to_backwards(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|row| row.iter().copied().rev().collect())
        .collect()
}

fn to_horizontal(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    input.iter().map(|row| row.to_vec()).collect()
}

fn solve(input: &Vec<Vec<char>>) -> usize {
    let input_horizontal = to_horizontal(input);
    let input_backwards = to_backwards(input);
    let input_vertical = to_vertical(input);
    let input_vertical_backwards = to_vertical_backwards(input);
    let input_top_left_bottom_right = to_top_left_bottom_right(input);
    let input_top_right_bottom_left = to_top_right_bottom_left(input);
    let input_bottom_left_top_right = to_bottom_left_top_right(input);
    let input_bottom_right_top_left = to_bottom_right_top_left(input);

    let scenarios = vec![
        input_horizontal,
        input_backwards,
        input_vertical,
        input_vertical_backwards,
        input_top_left_bottom_right,
        input_top_right_bottom_left,
        input_bottom_left_top_right,
        input_bottom_right_top_left,
    ];

    let target = "XMAS";

    scenarios
        .iter()
        .map(|scenario| {
            scenario
                .iter()
                .map(|row| {
                    row.windows(target.len())
                        .filter(|window| window.iter().collect::<String>() == target)
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = read_input();
    let solution = solve(&input);
    println!("{}", solution);
}
