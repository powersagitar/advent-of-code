use multimap::MultiMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (MultiMap<u32, u32>, Vec<Vec<u32>>) {
    let file = File::open("input.txt").unwrap();
    // let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules: MultiMap<u32, u32> = MultiMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut reached_blank_line = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            reached_blank_line = true;
            continue;
        }

        if !reached_blank_line {
            let x_y: Vec<u32> = line.split('|').map(|x| x.parse().unwrap()).collect();
            let x = x_y[0];
            let y = x_y[1];
            rules.insert(x, y);
        } else {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    (rules, updates)
}

fn is_in_right_order(rules: &MultiMap<u32, u32>, update: &[u32]) -> bool {
    let mut allowed_pages = HashSet::new();
    allowed_pages.insert(update[0]);

    for page in update {
        if !allowed_pages.contains(page) {
            return false;
        }

        // this line has potential problems
        // maybe not the entire map is to be cleared
        // im expecting a bug here but apprently it works
        allowed_pages.clear();

        // println!("page: {}", page);

        match rules.get_vec(page) {
            Some(rule) => {
                rule.iter().for_each(|&x| {
                    allowed_pages.insert(x);
                });
            }
            None => {
                continue;
            }
        }
    }

    true
}

fn solve(rules: &MultiMap<u32, u32>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|update| is_in_right_order(rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    let (rules, updates) = read_input();
    // println!("rules: {:?}", rules);
    // println!("updates: {:?}", updates);
    let solution = solve(&rules, &updates);
    println!("solution: {}", solution);
}