use linked_hash_map::LinkedHashMap;

use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    input.trim().split(',').map(hash).sum()
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();

    let mut boxes = vec![LinkedHashMap::<&str, usize>::new(); 256];
    input.trim().split(',').for_each(|s| {
        if let Some(label) = s.strip_suffix('-') {
            let box_id = hash(label);
            boxes[box_id].remove(label);
        } else {
            let (label, lens) = s.split_once('=').expect("an assignment");
            let lens = lens.parse::<usize>().expect("a number");
            let box_id = hash(label);
            let lenses = &mut boxes[box_id];
            if lenses.contains_key(label) {
                lenses.entry(label).and_modify(|l| *l = lens);
            } else {
                lenses.insert(label, lens);
            }
        }
    });

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_index, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(lens_index, (_label, lens))| (box_index + 1) * (lens_index + 1) * lens)
        })
        .sum()
}

fn hash(input: &str) -> usize {
    let mut current_value = 0;
    input.chars().for_each(|c| {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing() {
        assert_eq!(52, hash("HASH"));
        assert_eq!(30, hash("rn=1"));
        assert_eq!(0, hash("rn"));
        assert_eq!(1, hash("qp"));
        assert_eq!(3, hash("pc"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(1320, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(515974, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(145, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(265894, part2("input.txt"));
    }
}
