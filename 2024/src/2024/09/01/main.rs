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

fn find_last_file(disk: &[DiskNode]) -> usize {
    disk.iter()
        .rposition(|&node| matches!(node, DiskNode::File(_)))
        .unwrap()
}

fn solve(mut disk: Vec<DiskNode>) -> usize {
    let mut last_file = find_last_file(&disk);

    loop {
        let first_free_space = disk
            .iter()
            .position(|&node| matches!(node, DiskNode::FreeSpace))
            .unwrap();

        disk.swap(first_free_space, last_file);

        last_file = find_last_file(&disk);

        if !disk[..=last_file].contains(&DiskNode::FreeSpace) {
            break;
        }
    }

    disk[..=last_file]
        .into_iter()
        .enumerate()
        .map(|(pos, &node)| match node {
            DiskNode::FreeSpace => unreachable!(),
            DiskNode::File(id) => pos * id,
        })
        .sum()
}

fn main() {
    let input = read_input();
    // println!("{:?}", input);

    let solution = solve(input.clone());
    println!("solution: {}", solution);
}
