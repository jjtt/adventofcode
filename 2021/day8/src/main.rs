use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn x_from_input(input: &str) -> Vec<Vec<Vec<String>>> {
    read_to_string(input)
        .unwrap()
        .trim()
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(l: &str) -> Vec<Vec<String>> {
    l.trim()
        .split("|")
        .map(str::to_string)
        .map(|s| {
            s.trim()
                .split(" ")
                .map(|s| s.chars().sorted().collect::<String>())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use test_case::test_case;

    use super::*;

    #[test]
    fn single() {
        let l = parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        let inputs = l.get(0).unwrap();
        let outputs = l.get(1).unwrap();

        let known = foo(inputs);

        assert_eq!(2, *known.get("acdfg").unwrap());

        assert_eq!(
            5353,
            outputs
                .iter()
                .map(|x| known.get(x).unwrap().to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i32>()
                .unwrap()
        );
    }

    fn foo(inputs: &Vec<String>) -> HashMap<String, i32> {
        let mut known = HashMap::new();

        let easies = easy_mappings(inputs);

        for (s, v) in &easies {
            known.insert(*v, s.clone());
        }

        let mut remaining: Vec<String> = inputs
            .iter()
            .filter(|i| !easies.contains_key(*i))
            .map(|s| s.to_string())
            .collect();
        known.insert(
            9,
            remaining
                .iter()
                .filter(|i| i.len() == 6)
                .filter(|s| {
                    let mut s = (*s).clone();
                    s.retain(|x| !known.get(&(4)).unwrap().contains(x));
                    s.retain(|x| !known.get(&(7)).unwrap().contains(x));
                    s.len() == 1
                })
                .map(|s| s.to_string())
                .next()
                .unwrap(),
        );
        remaining.retain(|x| x != known.get(&(9)).unwrap());
        known.insert(
            0,
            remaining
                .iter()
                .filter(|i| i.len() == 6)
                .filter(|s| {
                    let mut s = (*s).clone();
                    s.retain(|x| !known.get(&(1)).unwrap().contains(x));
                    s.len() == 4
                })
                .map(|s| s.to_string())
                .last()
                .unwrap(),
        );
        remaining.retain(|x| x != known.get(&(0)).unwrap());
        known.insert(
            6,
            remaining
                .iter()
                .filter(|i| i.len() == 6)
                .map(|s| s.to_string())
                .last()
                .unwrap(),
        );
        remaining.retain(|x| x != known.get(&(6)).unwrap());
        known.insert(
            3,
            remaining
                .iter()
                .filter(|s| {
                    let mut s = (*s).clone();
                    s.retain(|x| !known.get(&(1)).unwrap().contains(x));
                    s.len() == 3
                })
                .map(|s| s.to_string())
                .last()
                .unwrap(),
        );
        remaining.retain(|x| x != known.get(&(3)).unwrap());
        known.insert(
            5,
            remaining
                .iter()
                .filter(|s| {
                    let mut s = (*s).clone();
                    s.retain(|x| !known.get(&(9)).unwrap().contains(x));
                    s.len() == 0
                })
                .map(|s| s.to_string())
                .last()
                .unwrap(),
        );
        remaining.retain(|x| x != known.get(&(5)).unwrap());
        known.insert(2, remaining.get(0).unwrap().clone());

        let mut out = HashMap::new();
        for (v, s) in known {
            out.insert(s, v);
        }

        out
    }

    fn easy_mappings(input: &Vec<String>) -> HashMap<String, i32> {
        input
            .iter()
            .map(|l| match l.len() {
                7 => Some((l.clone(), 8)),
                4 => Some((l.clone(), 4)),
                3 => Some((l.clone(), 7)),
                2 => Some((l.clone(), 1)),
                _ => None,
            })
            .filter_map(|x| x)
            .collect()
    }

    #[test_case("sample1.txt" => is eq(26) ; "sample")]
    #[test_case("input.txt" => is eq(479) ; "input")]
    fn part1(input: &str) -> usize {
        let x = x_from_input(input);

        let mut sum: usize = 0;
        let easy_ones = vec![7, 4, 3, 2];
        for l in x {
            let output = l.get(1).unwrap();
            sum += output
                .iter()
                .map(String::len)
                .filter(|o| easy_ones.contains(&o))
                .count();
        }

        sum
    }

    #[test_case("sample1.txt" => is eq(61229) ; "sample")]
    #[test_case("input.txt" => is eq(1041746) ; "input")]
    fn part2(input: &str) -> i32 {
        let x = x_from_input(input);

        let mut sum = 0;

        for l in x {
            let inputs = l.get(0).unwrap();
            let outputs = l.get(1).unwrap();

            let known = foo(inputs);

            sum += outputs
                .iter()
                .map(|x| known.get(x).unwrap().to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i32>()
                .unwrap();
        }

        sum
    }
}
