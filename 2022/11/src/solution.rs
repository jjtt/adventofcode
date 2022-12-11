use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;
use std::str::FromStr;

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    divider: i64,
    target_true: usize,
    target_false: usize,
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
        })
    }
}

pub fn part1(input: &str) -> i32 {
    //todo!()
    0
}

pub fn part2(input: &str) -> i32 {
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
        assert_eq!(vec![79, 98], monkey.items);
        assert_eq!(19, (monkey.operation)(1));
        assert_eq!(23, monkey.divider);
        assert_eq!(2, monkey.target_true);
        assert_eq!(3, monkey.target_false);

        let monkey = monkeys.last().unwrap();
        assert_eq!(vec![74], monkey.items);
        assert_eq!(4, (monkey.operation)(1));
        assert_eq!(17, monkey.divider);
        assert_eq!(0, monkey.target_true);
        assert_eq!(1, monkey.target_false);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(10605, part1("sample.txt"));
    }
}
