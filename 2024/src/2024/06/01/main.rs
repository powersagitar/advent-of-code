use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bearing {
    Up,
    Down,
    Left,
    Right,
}

impl Add<isize> for Bearing {
    type Output = Bearing;

    fn add(self, rhs: isize) -> Bearing {
        let mut new_bearing = self;

        for _ in 0..rhs {
            new_bearing = match new_bearing {
                Bearing::Up => Bearing::Right,
                Bearing::Right => Bearing::Down,
                Bearing::Down => Bearing::Left,
                Bearing::Left => Bearing::Up,
            };
        }

        new_bearing
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CoordinateType {
    Clear,
    Obstacle,
    Guard(Bearing),
}

fn read_input() -> Vec<Vec<CoordinateType>> {
    // let file = File::open("test.txt").unwrap();
    let file = File::open("input.txt").unwrap();
    let input = BufReader::new(file);

    let mut map: Vec<Vec<CoordinateType>> = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        let row: Vec<CoordinateType> = line
            .chars()
            .map(|c| match c {
                '.' => CoordinateType::Clear,
                '#' => CoordinateType::Obstacle,
                '^' => CoordinateType::Guard(Bearing::Up),
                _ => panic!("Invalid character in input"),
            })
            .collect();

        map.push(row);
    }

    map
}

fn simulate_guard(map: &Vec<Vec<CoordinateType>>) -> Vec<Vec<bool>> {
    let (mut guard_row, mut guard_col, mut guard_bearing) = (0, 0, Bearing::Up);

    let mut guard_path: Vec<Vec<bool>> = map
        .into_iter()
        .enumerate()
        .map(|(row_number, row)| {
            row.iter()
                .enumerate()
                .map(|(col_number, coord_t)| {
                    if let CoordinateType::Guard(_) = coord_t {
                        guard_row = row_number;
                        guard_col = col_number;
                        guard_bearing = Bearing::Up;
                    }

                    false
                })
                .collect()
        })
        .collect();

    loop {
        guard_path[guard_row][guard_col] = true;

        let (new_guard_row, new_guard_col) = match guard_bearing {
            Bearing::Up => {
                if guard_row == 0 {
                    break;
                }

                (guard_row - 1, guard_col)
            }
            Bearing::Down => {
                if guard_row == map.len() - 1 {
                    break;
                }

                (guard_row + 1, guard_col)
            }
            Bearing::Left => {
                if guard_col == 0 {
                    break;
                }

                (guard_row, guard_col - 1)
            }
            Bearing::Right => {
                if guard_col == map[guard_row].len() - 1 {
                    break;
                }

                (guard_row, guard_col + 1)
            }
        };

        if map[new_guard_row][new_guard_col] == CoordinateType::Obstacle {
            guard_bearing = guard_bearing + 1;
            continue;
        }

        guard_row = new_guard_row;
        guard_col = new_guard_col;
    }

    guard_path
}

fn solve(map: &Vec<Vec<CoordinateType>>) -> usize {
    let guard_path = simulate_guard(map);

    guard_path
        .iter()
        .map(|row| row.iter().filter(|&&is_guard| is_guard).count())
        .sum()
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<CoordinateType>>) {
    println!("Map:");
    map.iter().for_each(|row| println!("{:?}", row));
}

fn main() {
    let map = read_input();
    // print_map(&map);
    let solution = solve(&map);
    println!("Solution: {}", solution);
}
