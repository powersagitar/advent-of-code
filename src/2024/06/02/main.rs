use std::{
    collections::HashMap,
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

static mut OBSTRUCTION_PROBABILITY: usize = 1;

fn guard_is_trapped(map: &Vec<Vec<CoordinateType>>) -> bool {
    let (mut guard_row, mut guard_col, mut guard_bearing) = || -> (usize, usize, Bearing) {
        for (row_number, row) in map.iter().enumerate() {
            for (col_number, coord_t) in row.iter().enumerate() {
                if let CoordinateType::Guard(bearing) = coord_t {
                    return (row_number, col_number, bearing.clone());
                }
            }
        }

        return (0, 0, Bearing::Up);
    }();
    let mut occurrences: HashMap<(usize, usize), usize> = HashMap::new();

    unsafe {
        println!("obstruction probability {}", OBSTRUCTION_PROBABILITY);
        OBSTRUCTION_PROBABILITY += 1;
    }

    loop {
        occurrences
            .entry((guard_row, guard_col))
            .and_modify(|e| *e += 1)
            .or_insert(1);

        if occurrences.iter().filter(|(_, &count)| count > 100).count() > 4 {
            return true;
        }

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

    false
}

fn solve(map: &Vec<Vec<CoordinateType>>) -> usize {
    (0..map.len())
        .map(|row| {
            (0..map[row].len())
                .map(|col| {
                    let mut map = map.clone();
                    map[row][col] = CoordinateType::Obstacle;
                    map
                })
                .filter(|map| guard_is_trapped(&map))
                .count()
        })
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
