#[macro_use]
extern crate cached;
use cached::UnboundCache;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn game_from_input(input: &str) -> (String, HashMap<String, String>) {
    let string = read_to_string(input).unwrap();
    let mut lines = string.lines();

    (
        lines.next().unwrap().to_string(),
        lines
            .skip(1)
            .map(|r| {
                r.split(" -> ")
                    .map(|r| r.to_string())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
    )
}

fn count(polymer: String) -> HashMap<char, usize> {
    let counts = polymer.chars().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0) += 1;
        m
    });
    counts
}

fn count_most_least(counts: HashMap<char, usize>) -> (usize, usize) {
    let counts = counts;
    (
        *counts.values().max().unwrap(),
        *counts.values().min().unwrap(),
    )
}

fn apply(template: String, rules: &HashMap<String, String>) -> String {
    let mut polymer = template[..1].to_string();

    for i in 0..template.len() - 1 {
        let pair = &template[i..=i + 1];
        let last = &pair[1..];
        match rules.get(pair) {
            Some(insert) => {
                polymer += insert;
                polymer += last
            }
            None => polymer += last,
        }
    }

    polymer
}

fn merge(counts: &mut HashMap<char, usize>, more: &HashMap<char, usize>) {
    for (c, n) in more {
        *counts.entry(*c).or_insert(0) += *n;
    }
}

cached_key! {
    // name and type
    NAME_OF_CACHE: UnboundCache<String, HashMap<char, usize>> = UnboundCache::new();
    // construct key, cache is a shared one, so include rules in key
    Key = { format!("{}{}{}", pair, rounds, rules.len())};
    fn apply_rec(rules: &HashMap<String, String>, pair: &str, rounds: i32) -> HashMap<char, usize> = {
        let mut counts = HashMap::new();
        if rounds > 0 {
            let mut first = pair[..1].to_string();
            let mut last = pair[1..].to_string();
            let new = rules.get(pair).unwrap().chars().next().unwrap();
            first.push(new);
            last.insert(0, new);
            *counts.entry(new).or_insert(0) +=1;
            merge(&mut counts, &apply_rec(&rules, &first, rounds-1));
            merge(&mut counts, &apply_rec(&rules, &last, rounds-1));
        }
        counts
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(1588) ; "sample1")]
    #[test_case("input.txt" => is eq(3306) ; "input")]
    fn part1(input: &str) -> usize {
        let (template, rules) = game_from_input(input);

        let mut polymer = template;
        for _ in 0..10 {
            polymer = apply(polymer, &rules);
        }

        let (most, least) = count_most_least(count(polymer));

        most - least
    }

    #[test_case("sample1.txt" => is eq(2188189693529) ; "sample1")]
    #[test_case("input.txt" => is eq(3760312702877) ; "input")]
    fn part2(input: &str) -> usize {
        let (template, rules) = game_from_input(input);

        let mut counts = count(template.clone());

        for i in 0..&template.len() - 1 {
            let pair = &template[i..=i + 1];
            merge(&mut counts, &apply_rec(&rules, pair, 40));
        }

        let (most, least) = count_most_least(counts);

        most - least
    }
}
