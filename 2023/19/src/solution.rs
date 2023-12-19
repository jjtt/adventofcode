use std::collections::HashMap;
use std::fs::read_to_string;

use scan_fmt::scan_fmt;

enum Rule {
    Noop(String),
    GreaterThan(char, usize, String),
    LessThan(char, usize, String),
}

type Part = HashMap<char, usize>;

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let input = read_to_string(input).unwrap();
    let (rules, parts) = input.trim().split_once("\n\n").expect("a blank line");
    let rules = rules
        .lines()
        .map(|line| {
            let (label, rules) = scan_fmt!(line, "{}{{{}}}", String, String).expect("a rule line");
            (
                label,
                rules
                    .split(',')
                    .map(|rule| {
                        if rule.contains(':') {
                            let (condition, target) = rule.split_once(':').expect("a rule");
                            let category = condition[0..1].chars().next().expect("a category");
                            let op = &condition[1..2];
                            let value = condition[2..].parse::<usize>().expect("a number");
                            match op {
                                ">" => Rule::GreaterThan(category, value, target.to_string()),
                                "<" => Rule::LessThan(category, value, target.to_string()),
                                _ => panic!("unknown op: {op}"),
                            }
                        } else {
                            Rule::Noop(rule.to_string())
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    let parts = parts
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(',')
                .map(|category| scan_fmt!(category, "{}={}", char, usize).expect("a category"))
                .collect()
        })
        .collect();

    (rules, parts)
}

fn apply(rules: &HashMap<String, Vec<Rule>>, part: &Part, label: &str) -> bool {
    match label {
        "A" => true,
        "R" => false,
        label => {
            let rule = rules.get(label).expect("a rule");

            for rule in rule {
                match rule {
                    Rule::Noop(target) => return apply(rules, part, target),
                    Rule::GreaterThan(category, value, target) if part[category] > *value => {
                        return apply(rules, part, target)
                    }
                    Rule::LessThan(category, value, target) if part[category] < *value => {
                        return apply(rules, part, target)
                    }
                    _ => {}
                }
            }

            panic!("no rule applied for {label} and {part:?}");
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (rules, parts) = parse_input(input);

    parts
        .into_iter()
        .filter(|part| apply(&rules, part, "in"))
        .map(|part| part[&'x'] + part[&'m'] + part[&'a'] + part[&'s'])
        .sum()
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
        assert_eq!(19114, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(495298, part1("input.txt"));
    }
}
