use std::collections::HashSet;
use std::fs::read_to_string;

enum Block {
    File(i32, usize, usize),
    Empty(usize, usize),
}

fn parse(input: &str, small_blocks: bool) -> Vec<Block> {
    let input = read_to_string(input).unwrap();
    let input = input.trim();

    let mut disk = Vec::new();

    let mut index = 0;
    for (i, c) in input.chars().enumerate() {
        let size = c as usize - '0' as usize;
        let is_file = i % 2 == 0;
        let id = i as i32 / 2;
        if small_blocks {
            for _ in 0..size {
                let size = 1;
                disk.push(if is_file {
                    Block::File(id, index, size)
                } else {
                    Block::Empty(index, size)
                });
                index += size;
            }
        } else {
            disk.push(if is_file {
                Block::File(id, index, size)
            } else {
                Block::Empty(index, size)
            });
            index += size;
        }
    }

    disk.truncate(index);
    disk
}

pub fn part1(input: &str) -> usize {
    let disk = parse(input, true);

    let mut checksum = 0;
    let mut i = 0;
    let mut last = disk.len() - 1;
    while i <= last {
        match (&disk[i], &disk[last]) {
            (Block::Empty(_, 1), Block::Empty(_, 1)) => {
                last -= 1;
            }
            (Block::Empty(_, 1), Block::File(id, _, 1)) => {
                checksum += *id as usize * i;
                last -= 1;
                i += 1;
            }
            (Block::File(id, _, 1), _) => {
                checksum += *id as usize * i;
                i += 1;
            }
            _ => panic!("Not implemented"),
        }
    }

    checksum
}

pub fn part2(input: &str) -> usize {
    let disk = parse(input, false);

    let mut moved_files = HashSet::new();
    let mut defragmented = Vec::new();
    for i in 0..disk.len() {
        match &disk[i] {
            Block::Empty(mut free_space_index, mut free_space_size) => {
                for j in (i..disk.len()).rev() {
                    match &disk[j] {
                        Block::File(id, _, file_size)
                            if *file_size <= free_space_size && !moved_files.contains(&id) =>
                        {
                            defragmented.push(Block::File(*id, free_space_index, *file_size));
                            moved_files.insert(id);
                            free_space_index += file_size;
                            free_space_size -= file_size;
                            assert!(free_space_size >= 0);
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                if free_space_size > 0 {
                    defragmented.push(Block::Empty(free_space_index, free_space_size));
                }
            }
            Block::File(id, file_index, file_size) if !moved_files.contains(id) => {
                defragmented.push(Block::File(*id, *file_index, *file_size));
            }
            Block::File(id, _, _) => {
                assert!(moved_files.contains(id));
                continue;
            }
        }
    }

    let mut checksum = 0;
    for block in defragmented {
        if let Block::File(id, index, size) = block {
            for i in index..index + size {
                checksum += id as usize * i;
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(1928, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6353658451014, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(2858, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(6382582136592, part2("input.txt"));
    }
}
