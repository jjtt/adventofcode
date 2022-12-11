use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Barrel {
    monkeys: Vec<Monkey>,
}

impl Barrel {
    fn do_throws(&mut self) {
        for m in 0..self.monkeys.len() {
            for (target, item) in self.do_monkey(m) {
                self.monkeys[target].items.push_back(item);
            }
        }
    }

    fn do_monkey(&mut self, m: usize) -> Vec<(usize, i64)> {
        let mut throws = vec![];
        let monkey = self.monkeys.get_mut(m).unwrap();
        while let Some(throw) = monkey.throw_one() {
            throws.push(throw);
        }
        throws
    }

    fn monkey_business(&self) -> usize {
        let mut monkeys = self
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<usize>>();
        monkeys.sort();
        monkeys[monkeys.len() - 1] * monkeys[monkeys.len() - 2]
    }
}

struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    divider: i64,
    target_true: usize,
    target_false: usize,
    inspections: usize,
}

impl Monkey {
    fn parse_operation(input: &str) -> Box<dyn Fn(i64) -> i64> {
        if let Ok(x) = scan_fmt!(input, "old + {d}", i64) {
            Box::new(move |old| old + x)
        } else if let Ok(x) = scan_fmt!(input, "old * {d}", i64) {
            Box::new(move |old| old * x)
        } else if input == "old * old" {
            Box::new(move |old| old * old)
        } else {
            panic!("Unsupported operation: {input}");
        }
    }

    fn throw_one(&mut self) -> Option<(usize, i64)> {
        if let Some(item) = self.items.pop_front() {
            self.inspections += 1;
            let item = (self.operation)(item);
            let item = item / 3;
            let target = if item % self.divider == 0 {
                self.target_true
            } else {
                self.target_false
            };
            Some((target, item))
        } else {
            None
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        assert!(lines.next().unwrap().starts_with("Monkey "));
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let operation = Self::parse_operation(
            lines
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = ")
                .unwrap(),
        );
        let divider = scan_fmt!(lines.next().unwrap(), "Test: divisible by {d}", i64).unwrap();
        let target_true = scan_fmt!(
            lines.next().unwrap(),
            "  If true: throw to monkey {d}",
            usize
        )
        .unwrap();
        let target_false = scan_fmt!(
            lines.next().unwrap(),
            "  If false: throw to monkey {d}",
            usize
        )
        .unwrap();

        Ok(Monkey {
            items,
            operation,
            divider,
            target_true,
            target_false,
            inspections: 0,
        })
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

pub fn part1(input: &str) -> usize {
    let mut barrel = Barrel {
        monkeys: parse(&read_to_string(input).unwrap()),
    };

    for m in (0..20) {
        barrel.do_throws();
    }

    barrel.monkey_business()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .into_iter()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_sample() {
        let monkeys = parse(&read_to_string("sample.txt").unwrap());
        assert_eq!(4, monkeys.len());

        let monkey = monkeys.first().unwrap();
        assert_eq!(vec![79, 98], monkey.items.clone().make_contiguous());
        assert_eq!(19, (monkey.operation)(1));
        assert_eq!(23, monkey.divider);
        assert_eq!(2, monkey.target_true);
        assert_eq!(3, monkey.target_false);

        let monkey = monkeys.last().unwrap();
        assert_eq!(vec![74], monkey.items.clone().make_contiguous());
        assert_eq!(4, (monkey.operation)(1));
        assert_eq!(17, monkey.divider);
        assert_eq!(0, monkey.target_true);
        assert_eq!(1, monkey.target_false);
    }

    #[test]
    fn monkey_business() {
        let barrel = Barrel {
            monkeys: vec![
                Monkey {
                    items: VecDeque::new(),
                    operation: Box::new(|x| x),
                    divider: 0,
                    target_true: 0,
                    target_false: 0,
                    inspections: 20,
                },
                Monkey {
                    items: VecDeque::new(),
                    operation: Box::new(|x| x),
                    divider: 0,
                    target_true: 0,
                    target_false: 0,
                    inspections: 10,
                },
                Monkey {
                    items: VecDeque::new(),
                    operation: Box::new(|x| x),
                    divider: 0,
                    target_true: 0,
                    target_false: 0,
                    inspections: 30,
                },
            ],
        };

        assert_eq!(600, barrel.monkey_business());
    }

    #[test]
    fn throw_one() {
        let mut monkey = Monkey {
            items: VecDeque::from(vec![1]),
            operation: Box::new(|x| x * 4),
            divider: 1,
            target_true: 42,
            target_false: 0,
            inspections: 0,
        };

        let (target, item) = monkey.throw_one().unwrap();

        assert_eq!(42, target);
        assert_eq!(1, item);
        assert_eq!(1, monkey.inspections);
        assert!(monkey.items.is_empty())
    }

    #[test]
    fn part1_sample() {
        assert_eq!(10605, part1("sample.txt"));
    }
}
