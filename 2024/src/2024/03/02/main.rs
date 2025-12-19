use std::fs::read_to_string;

use regex::Regex;

fn read_input() -> String {
    read_to_string("input.txt").unwrap().replace('\n', "")
}

fn solve(input: &str) -> i32 {
    let input = input.to_owned() + "do()";
    // regex doesn't match string after don't() without a closing do()
    let input = Regex::new(r"don't\(\).*?do\(\)")
        .unwrap()
        .replace_all(&input, "");

    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for (_full, [a, b]) in mul_pattern.captures_iter(&input).map(|c| c.extract()) {
        sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    sum
}

fn main() {
    let input = read_input();
    let solution = solve(&input);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve(input), 48);
    }
}
