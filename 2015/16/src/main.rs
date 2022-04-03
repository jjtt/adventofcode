use single::Single;
use std::collections::HashMap;
use std::fs::read_to_string;

fn filter_part1(sue: &HashMap<String, i32>, evidence: &HashMap<String, i32>) -> bool {
    for (e, n) in evidence {
        if sue.contains_key(e) {
            if sue[e] != *n {
                return false;
            }
        }
    }
    true
}

fn filter_part2(sue: &HashMap<String, i32>, evidence: &HashMap<String, i32>) -> bool {
    for (e, n) in evidence {
        if sue.contains_key(e) {
            if match e.as_str() {
                "cats" | "trees" => sue[e] <= *n,
                "pomeranians" | "goldfish" => sue[e] >= *n,
                _ => sue[e] != *n,
            } {
                return false;
            }
        }
    }
    true
}

fn filter_sues(
    input: String,
    evidence: &HashMap<String, i32>,
    filter: fn(&HashMap<String, i32>, &HashMap<String, i32>) -> bool,
) -> (i32, HashMap<String, i32>) {
    input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(s, p)| {
            (
                s[4..].parse::<i32>().unwrap(),
                p.split(", ")
                    .map(|p| p.split_once(": ").unwrap())
                    .map(|(p, n)| (p.to_string(), n.parse::<i32>().unwrap()))
                    .collect::<HashMap<String, i32>>(),
            )
        })
        .filter(|(_, s)| filter(&s, &evidence))
        .single()
        .unwrap()
}

fn main() {
    let evidence: HashMap<String, i32> = read_to_string("evidence.txt")
        .unwrap()
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(e, n)| (e.to_string(), n.parse::<i32>().unwrap()))
        .collect();

    let input = read_to_string("input.txt").unwrap();
    let sue = filter_sues(input.clone(), &evidence, filter_part1);

    println!("Part 1 - Sue {}", sue.0);

    let sue = filter_sues(input, &evidence, filter_part2);

    println!("Part 2 - Sue {}", sue.0);
}
