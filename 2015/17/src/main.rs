use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let test_containers = vec![20, 15, 10, 5, 5];

    let containers = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    println!("Test - {} combinations", combinations(25, &test_containers));
    println!("Part1 - {} combinations", combinations(150, &containers));
}

fn combinations(nog: i32, containers: &Vec<i32>) -> usize {
    (1..containers.len())
        .map(|size| {
            containers
                .iter()
                .combinations(size)
                .filter(|combination| nog == combination.iter().copied().sum())
                .count()
        })
        .sum()
}
