use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

use scan_fmt::scan_fmt;

enum Rule {
    Accept,
    Reject,
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
                            match rule {
                                "A" => Rule::Accept,
                                "R" => Rule::Reject,
                                rule => Rule::Noop(rule.to_string()),
                            }
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
                    Rule::Accept => return true,
                    Rule::Reject => return false,
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

fn count_accepted(
    rules: &HashMap<String, Vec<Rule>>,
    label: &str,
    mut x: RangeInclusive<usize>,
    mut m: RangeInclusive<usize>,
    mut a: RangeInclusive<usize>,
    mut s: RangeInclusive<usize>,
) -> usize {
    dbg!(label, &x, &m, &a, &s);
    let total = x.clone().count() * m.clone().count() * a.clone().count() * s.clone().count();
    let count = match label {
        "A" => total,
        "R" => 0,
        label => {
            let rule = rules.get(label).expect("a rule");

            let mut sum = 0;
            for r in rule {
                match r {
                    Rule::Accept => {
                        sum += (x.clone().count()
                            * m.clone().count()
                            * a.clone().count()
                            * s.clone().count())
                    }
                    Rule::Reject => sum += 0,
                    Rule::Noop(target) => {
                        sum += count_accepted(
                            rules,
                            target,
                            x.clone(),
                            m.clone(),
                            a.clone(),
                            s.clone(),
                        )
                    }
                    Rule::GreaterThan(category, value, target) => match category {
                        'x' => {
                            sum += count_accepted(
                                rules,
                                target,
                                (value + 1)..=*x.end(),
                                m.clone(),
                                a.clone(),
                                s.clone(),
                            );
                            x = *x.start()..=*value;
                        }
                        'm' => {
                            sum += count_accepted(
                                rules,
                                target,
                                x.clone(),
                                (value + 1)..=*m.end(),
                                a.clone(),
                                s.clone(),
                            );
                            m = *m.start()..=*value;
                        }
                        'a' => {
                            sum += count_accepted(
                                rules,
                                target,
                                x.clone(),
                                m.clone(),
                                (value + 1)..=*a.end(),
                                s.clone(),
                            );
                            a = *a.start()..=*value;
                        }
                        's' => {
                            sum += count_accepted(
                                rules,
                                target,
                                x.clone(),
                                m.clone(),
                                a.clone(),
                                (value + 1)..=*s.end(),
                            );
                            s = *s.start()..=*value;
                        }
                        _ => panic!("unknown category: {category}"),
                    },
                    Rule::LessThan(category, value, target) => match category {
                        'x' => {
                            sum += count_accepted(
                                rules,
                                target,
                                *x.start()..=(value - 1),
                                m.clone(),
                                a.clone(),
                                s.clone(),
                            );
                            x = *value..=*x.end();
                        }
                        'm' => {
                            sum += count_accepted(
                                rules,
                                target,
                                x.clone(),
                                *m.start()..=(value - 1),
                                a.clone(),
                                s.clone(),
                            );
                            m = *value..=*m.end();
                        }
                        'a' => {
                            sum += count_accepted(
                                rules,
                                target,
                                x.clone(),
                                m.clone(),
                                *a.start()..=(value - 1),
                                s.clone(),
                            );
                            a = *value..=*a.end();
                        }
                        's' => {
                            sum += count_accepted(
                                rules,
                                target,
                                x.clone(),
                                m.clone(),
                                a.clone(),
                                *s.start()..=(value - 1),
                            );
                            s = *value..=*s.end();
                        }
                        _ => panic!("unknown category: {category}"),
                    },
                }
            }
            sum
        }
    };
    dbg!(count)
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
    let (rules, _parts) = parse_input(input);

    count_accepted(&rules, "in", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_rule_pv() {
        let (rules, _parts) = parse_input("sample.txt");
        assert_eq!(
            1716 * 4000 * 4000 * 4000,
            count_accepted(&rules, "pv", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
        )
    }

    #[test]
    fn one_rule_lnx() {
        let (rules, _parts) = parse_input("sample.txt");
        assert_eq!(
            256000000000000,
            count_accepted(&rules, "lnx", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
        )
    }

    #[test]
    fn one_rule_crn() {
        let (rules, _parts) = parse_input("sample.txt");
        assert_eq!(
            (4000 - 2662) * 4000 * 4000 * 4000,
            count_accepted(&rules, "crn", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
        )
    }

    #[test]
    fn one_rule_gd() {
        let (rules, _parts) = parse_input("sample.txt");
        assert_eq!(
            0,
            count_accepted(&rules, "gd", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
        )
    }

    #[test]
    fn one_rule_rfg() {
        let (rules, _parts) = parse_input("sample.txt");
        assert_eq!(
            (4000 - 536) * 2440 * 4000 * 4000,
            count_accepted(&rules, "rfg", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
        )
    }

    #[test]
    fn one_rule_hdj() {
        let (rules, _parts) = parse_input("sample.txt");
        assert_eq!(
            (4000 - 838) * 4000 * 4000 * 4000 + 838 * 1716 * 4000 * 4000,
            count_accepted(&rules, "hdj", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
        )
    }

    #[test]
    fn part1_sample() {
        assert_eq!(19114, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(495298, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(167409079868000, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(132186256794011, part2("input.txt"));
    }
}
