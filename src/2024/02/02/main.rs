use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input() -> Vec<Vec<i32>> {
    let input = File::open("./input.txt").unwrap();
    // let input = File::open("./test.txt").unwrap();
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

fn check_safe(report: &[i32], tolerance: i32) -> bool {
    println!("report: {:?}", report);

    let expected_dir = (report[1] - report[0]).signum();

    println!("expected dir {expected_dir}");

    for i in 1..report.len() {
        let direction = (report[i] - report[i - 1]).signum();
        let dist = (report[i] - report[i - 1]).abs();

        if expected_dir != direction || dist > 3 || dist < 1 {
            if tolerance > 0 {
                if i == 1 {
                    return check_safe(
                        &[report[..i].to_vec(), report[i + 1..].to_vec()].concat(),
                        tolerance - 1,
                    ) || check_safe(&report[1..], tolerance - 1);
                } else {
                    return check_safe(
                        &[report[..i].to_vec(), report[i + 1..].to_vec()].concat(),
                        tolerance - 1,
                    );
                }
            } else {
                return false;
            }
        }
    }

    true
}

fn calc_safe_cnt(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_cnt = 0;

    for report in reports {
        if check_safe(&report, 1) {
            safe_cnt += 1;
        }
    }

    safe_cnt
}

fn main() {
    let reports = read_input();

    let safe_cnt = calc_safe_cnt(reports);

    println!("{}", safe_cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_payload() -> Vec<(Vec<i32>, bool)> {
        vec![
            (vec![18, 21, 26, 29, 32, 32], false),
            (vec![46, 48, 53, 54, 56, 58, 62], false),
            (vec![25, 23, 25, 28, 29, 32, 35], false),
            (vec![9, 8, 9, 12, 19], false),
            (vec![1, 3, 4, 5], true),
            (vec![5, 4, 3, 1], true),
            (vec![1, 5, 4], true),
            (vec![26, 28, 26, 23, 23, 22, 18], false),
            (vec![9, 9, 13, 16, 16], false),
            (vec![57, 52, 51, 48, 46, 48], false),
            (vec![33, 34, 38, 39, 40, 42], false),
            (vec![46, 48, 53, 54, 57, 58, 62], false),
            (vec![60, 59, 56, 55, 54, 52, 51], true),
            (vec![50, 52, 54, 60, 55], true),
            (vec![10, 1, 2, 3, 4, 5], true),
        ]

        // vec![(vec![10, 1, 2, 3, 4, 5], true)]
    }

    #[test]
    fn test_check_safe() {
        let reports = test_payload();

        for (report, is_safe) in reports {
            assert_eq!(check_safe(&report, 1), is_safe);
        }
    }

    #[test]
    fn test_calc_safe_cnt() {
        let payload = test_payload();

        assert_eq!(
            calc_safe_cnt(
                payload
                    .clone()
                    .into_iter()
                    .map(|(report, _)| report)
                    .collect()
            ),
            payload
                .clone()
                .into_iter()
                .map(|(_, is_safe)| if is_safe { 1 } else { 0 })
                .collect::<Vec<i32>>()
                .into_iter()
                .sum()
        );
    }
}
