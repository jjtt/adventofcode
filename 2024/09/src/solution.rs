use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let input = input.trim();

    let mut disk = vec![0; input.len() * 9];

    let mut index = 0;
    for (i, c) in input.chars().enumerate() {
        let size = c as usize - '0' as usize;
        let is_file = i % 2 == 0;
        let id = i as i32 / 2;
        for _ in 0..size {
            disk[index] = if is_file { id } else { -1i32 };
            index += 1;
        }
    }

    let disk = disk;

    for i in disk[0..index].iter() {
        if *i == -1 {
            print!(".");
        } else {
            print!("{}", i);
        }
    }
    println!();

    let mut checksum = 0;
    let mut i = 0;
    let mut last = index - 1;
    while i <= last {
        if disk[i] == -1 && disk[last] == -1 {
            last -= 1;
        } else if disk[i] == -1 {
            checksum += disk[last] as usize * i;
            last -= 1;
            i += 1;
        } else {
            checksum += disk[i] as usize * i;
            i += 1;
        }
    }

    checksum
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
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
}
