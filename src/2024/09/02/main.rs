use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiskNode {
    FreeSpace,
    File(usize),
}

fn read_input() -> Vec<DiskNode> {
    // let input = read_to_string("test.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    let mut is_file = true;
    let mut file_id = 0;

    input
        .trim()
        .chars()
        .map(|c| {
            if is_file {
                is_file = false;
                let size = c.to_digit(10).unwrap() as usize;
                file_id += 1;
                [DiskNode::File(file_id - 1)].repeat(size)
            } else {
                is_file = true;
                let size = c.to_digit(10).unwrap() as usize;
                [DiskNode::FreeSpace].repeat(size)
            }
        })
        .flatten()
        .collect()
}

fn find_last_file(disk: &[DiskNode]) -> (usize, usize) {
    let node = disk
        .iter()
        .rfind(|&node| matches!(node, DiskNode::File(_)))
        .unwrap();

    let id = match node {
        DiskNode::File(id) => *id,
        _ => unreachable!("last node is not a file"),
    };

    let end_idx_inclusive = disk
        .iter()
        .rposition(|node| matches!(node, &DiskNode::File(_)))
        .unwrap();

    let start_idx = disk
        .iter()
        .position(|&node| node == DiskNode::File(id))
        .unwrap();

    (start_idx, end_idx_inclusive + 1)
}

fn find_first_free_space(disk: &[DiskNode]) -> (usize, usize) {
    let start_idx = disk
        .iter()
        .position(|node| matches!(node, &DiskNode::FreeSpace))
        .unwrap();

    let mut end_idx = start_idx + 1;

    while end_idx < disk.len() && disk[end_idx] == DiskNode::FreeSpace {
        end_idx += 1;
    }

    (start_idx, end_idx)
}

fn solve(mut disk: Vec<DiskNode>) -> usize {
    let mut last_file = find_last_file(&disk);
    let mut first_free_space = find_first_free_space(&disk);

    while first_free_space.0 < last_file.0 {
        let mut free_space = first_free_space;

        while free_space.0 < last_file.0 {
            let free_space_size = free_space.1 - free_space.0;
            let file_size = last_file.1 - last_file.0;

            if free_space_size >= file_size {
                for i in 0..file_size {
                    disk.swap(free_space.0 + i, last_file.0 + i);
                }

                first_free_space = find_first_free_space(&disk);

                break;
            }

            free_space = {
                let free_space_in_slice = find_first_free_space(&disk[free_space.1..]);
                (
                    free_space_in_slice.0 + free_space.1,
                    free_space_in_slice.1 + free_space.1,
                )
            };
        }

        last_file = find_last_file(&disk[..last_file.0]);
    }

    disk.into_iter()
        .enumerate()
        .filter(|(_, node)| !matches!(node, DiskNode::FreeSpace))
        .map(|(pos, node)| match node {
            DiskNode::FreeSpace => unreachable!("free space in final disk"),
            DiskNode::File(id) => pos * id,
        })
        .sum()
}

fn main() {
    let input = read_input();

    let solution = solve(input.clone());
    println!("solution: {}", solution);
}
