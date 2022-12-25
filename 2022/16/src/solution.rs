use pathfinding::prelude::astar;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::hash::Hash;

const MAX_VALVES: usize = 25 * 26 + 25;

#[derive(Debug)]
struct Cave {}

impl Cave {
    fn new(mut valve_map: HashMap<usize, (Valve, Vec<usize>)>) -> Cave {
        todo!()
    }

    fn find_max_flow(&self, time: usize, count: usize) -> i64 {
        let result = astar(
            &SearchState::new(),
            |s| s.successors(self),
            |s| s.remaining(self),
            |s| s.done(self),
        );

        todo!()
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Valve {
    flow_rate: usize,
}

impl Valve {
    fn from(input: &str) -> (usize, (Valve, Vec<usize>)) {
        let input = input
            .replace("tunnels", "tunnel")
            .replace("leads", "lead")
            .replace("valves", "valve");
        let (name, flow_rate, tunnels) = scan_fmt!(
            &input,
            "Valve {} has flow rate={d}; tunnel lead to valve {/[A-Z, ]+/}",
            String,
            usize,
            String
        )
        .unwrap();

        let tunnels = tunnels.split(", ").map(name_to_int).collect();

        (name_to_int(&name), (Valve { flow_rate }, tunnels))
    }
}

fn name_to_int(name: &str) -> usize {
    let bytes = name.as_bytes();
    let first = bytes[0] - b'A';
    let second = bytes[1] - b'A';
    first as usize * 26 + second as usize
}

fn int_to_name(name: usize) -> String {
    let first = char::from((name / 26) as u8 + b'A');
    let second = char::from((name % 26) as u8 + b'A');

    String::from_iter([first, second])
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct SearchState {}

impl SearchState {
    fn new() -> SearchState {
        todo!()
    }

    fn successors(&self, cave: &Cave) -> Vec<(SearchState, i64)> {
        todo!()
    }

    fn remaining(&self, cave: &Cave) -> i64 {
        todo!()
    }

    fn done(&self, cave: &Cave) -> bool {
        todo!()
    }
}

pub fn part1(input: &str) -> i64 {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let time_available = 30;
    let cave = Cave::new(valves);

    cave.find_max_flow(time_available, 1)
}

pub fn part2(input: &str) -> i64 {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let time_available = 26;
    let cave = Cave::new(valves);

    cave.find_max_flow(time_available, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves BB"),
            Valve::from("Valve BB has flow rate=13; tunnels lead to valves AA"),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364, max);
    }

    #[test]
    fn simplish() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            Valve::from("Valve AA has flow rate=1; tunnels lead to valves BB"),
            Valve::from("Valve BB has flow rate=10; tunnels lead to valves AA"),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(28 * 10 + 26 * 1, max);
    }

    #[test]
    fn less_simple() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves BB"),
            Valve::from("Valve BB has flow rate=13; tunnels lead to valves AA, CC"),
            Valve::from("Valve CC has flow rate=2; tunnels lead to valves BB"),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364 + 52, max);
    }

    #[test]
    fn even_less_simple() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            Valve::from("Valve AA has flow rate=2; tunnels lead to valves BB"),
            Valve::from("Valve BB has flow rate=13; tunnels lead to valves AA, CC"),
            Valve::from("Valve CC has flow rate=100; tunnels lead to valves BB"),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(27 * 100 + 25 * 13 + 23 * 2, max);
    }

    #[test]
    fn still_less_simple() {
        let time_available = 10;
        let cave = Cave::new(HashMap::from([
            Valve::from("Valve AA has flow rate=2; tunnels lead to valves BB"),
            Valve::from("Valve BB has flow rate=13; tunnels lead to valves AA, CC"),
            Valve::from("Valve CC has flow rate=100; tunnels lead to valves BB"),
        ]));

        let max = cave.find_max_flow(time_available, 2);
        assert_eq!(7 * 100 + 8 * 13 + 6 * 2, max);
    }

    #[test]
    fn parsing() {
        assert_eq!(
            (
                name_to_int("AA"),
                (
                    Valve { flow_rate: 0 },
                    vec![name_to_int("DD"), name_to_int("II"), name_to_int("BB")]
                ),
            ),
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")
        );
    }

    #[test]
    fn name_to_index() {
        assert_eq!(0, name_to_int("AA"));
        assert_eq!(1, name_to_int("AB"));
        assert_eq!(2, name_to_int("AC"));
        assert_eq!(3, name_to_int("AD"));
        assert_eq!(25 * 26 + 23, name_to_int("ZX"));
    }

    #[test]
    fn index_to_name() {
        assert_eq!("AA", int_to_name(0));
        assert_eq!("BA", int_to_name(26));
        assert_eq!("ZZ", int_to_name(25 * 26 + 25));
    }

    #[test]
    fn name_to_index_to_name() {
        assert_eq!("ZX", int_to_name(name_to_int("ZX")));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(1651, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(1707, part2("sample.txt"));
    }
}
