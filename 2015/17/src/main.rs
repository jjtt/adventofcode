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
    println!("Part2 - {} minimum ways", min_ways(150, &containers));
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

fn min_ways(nog: i32, containers: &Vec<i32>) -> usize {
    let mut ways = (1..containers.len())
        .flat_map(|size| {
            containers
                .iter()
                .combinations(size)
                .filter(|combination| nog == combination.iter().copied().sum())
                .collect::<Vec<Vec<&i32>>>()
        })
        .collect::<Vec<Vec<&i32>>>();
    ways.sort_by_key(Vec::len);
    let min = ways.first().unwrap().len();
    for (index, way) in ways.iter().enumerate() {
        if way.len() != min {
            return index;
        }
    }
    panic!("Not found?")
}
