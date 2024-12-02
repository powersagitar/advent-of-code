use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input() -> Vec<Vec<i32>> {
    let input = File::open("./input.txt").unwrap();
    let input_reader = BufReader::new(input);

    let mut reports = Vec::new();

    for report in input_reader.lines() {
        let report = report.unwrap();

        reports.push(
            report
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect(),
        );
    }

    reports
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Asc,
    Dsc,
}

fn find_direction(diff: i32) -> Direction {
    if diff > 0 {
        Direction::Asc
    } else {
        Direction::Dsc
    }
}

fn main() {
    let reports = read_input();

    let mut safe_cnt = 0;

    for report in reports {
        let mut expected_direction: Option<Direction> = None;
        let mut is_safe = true;

        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];
            let diff_abs = diff.abs();
            let direction = find_direction(diff);

            if expected_direction.is_none() {
                expected_direction = Some(direction);
            }

            if expected_direction != Some(direction) || diff_abs > 3 || diff_abs < 1 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            println!("safe: {:?}", report);
            safe_cnt += 1;
        }
    }

    println!("{}", safe_cnt);
}
