use single::Single;
use std::collections::HashMap;
use std::fs::read_to_string;

fn filter(sue: &HashMap<String, i32>, evidence: &HashMap<String, i32>) -> bool {
    for (e, n) in evidence {
        if sue.contains_key(e) {
            if sue[e] != *n {
                return false;
            }
        }
    }
    true
}

fn main() {
    let evidence: HashMap<String, i32> = read_to_string("evidence.txt")
        .unwrap()
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(e, n)| (e.to_string(), n.parse::<i32>().unwrap()))
        .collect();
    dbg!(&evidence);

    let input = read_to_string("input.txt").unwrap();
    let sue = input
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
        .single();

    dbg!(sue);
}
