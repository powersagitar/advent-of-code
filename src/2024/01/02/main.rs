use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (Vec<usize>, HashMap<usize, usize>) {
    let input = File::open("src/2024/01/02/input.txt").unwrap();
    let input_reader = BufReader::new(input);

    let mut list1 = Vec::<usize>::new();
    let mut list2_occurrences = HashMap::<usize, usize>::new();

    for line in input_reader.lines() {
        let line = line.unwrap();

        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let num1 = nums[0];
        let num2 = nums[1];

        list1.push(num1);

        if list2_occurrences.contains_key(&num2) {
            let count = list2_occurrences.get_mut(&num2).unwrap();
            *count += 1;
        } else {
            list2_occurrences.insert(num2, 1);
        }
    }

    (list1, list2_occurrences)
}

fn main() {
    let (list1, list2_occurrences) = read_input();

    let len = list1.len();

    let mut total_similarity = 0;

    for i in 0..len {
        let num1 = list1[i];
        let list2_occurrence = list2_occurrences.get(&num1).unwrap_or(&0);

        let similarity = num1 * list2_occurrence;
        total_similarity += similarity;
    }

    println!("{}", total_similarity);
}
