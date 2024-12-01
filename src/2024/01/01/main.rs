use num::abs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (Vec<isize>, Vec<isize>) {
    let input = File::open("src/2024/01/01/input.txt").unwrap();
    let input_reader = BufReader::new(input);

    let mut list1 = Vec::<isize>::new();
    let mut list2 = Vec::<isize>::new();

    for line in input_reader.lines() {
        let line = line.unwrap();

        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        list1.push(nums[0]);
        list2.push(nums[1]);
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

fn main() {
    let (mut list1, mut list2) = read_input();

    let len = list1.len();

    let mut total_distance = 0;

    for _ in 0..len {
        let num1 = list1.pop().unwrap();
        let num2 = list2.pop().unwrap();

        let distance = abs(num1 - num2) as usize;
        total_distance += distance;
    }

    println!("{}", total_distance);
}
