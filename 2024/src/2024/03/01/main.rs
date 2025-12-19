use std::fs::read_to_string;

use regex::Regex;

fn read_input() -> String {
    read_to_string("input.txt").unwrap()
}

fn solve(input: &str) -> i32 {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for (_full, [a, b]) in mul_pattern.captures_iter(input).map(|c| c.extract()) {
        sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    sum
}

fn main() {
    let input = read_input();
    let solution = solve(&input);
    println!("{}", solution);
}
