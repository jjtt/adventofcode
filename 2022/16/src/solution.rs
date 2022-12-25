use itertools::Itertools;
use pathfinding::prelude::{astar, bfs_reach};
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

const MAX_VALVES: usize = 25 * 26 + 25;

#[derive(Debug)]
struct Cave {
    valves: [Option<Valve>; MAX_VALVES],
    neighbours: [[bool; MAX_VALVES]; MAX_VALVES],
}

impl Cave {
    fn new(mut valve_map: HashMap<usize, (Valve, Vec<usize>)>) -> Cave {
        let mut neighbours = [[false; MAX_VALVES]; MAX_VALVES];
        for (from, (_, to)) in &valve_map {
            for to in to {
                neighbours[*from][*to] = true;
            }
        }

        let mut valves = [None; MAX_VALVES];
        for name in 0..MAX_VALVES {
            valves[name] = valve_map.remove(&name).map(|(valve, _)| valve);
        }

        Cave { valves, neighbours }
    }

    fn successors(&self, current: &SearchState) -> impl IntoIterator<Item = (SearchState, i64)> {
        if current.time == 0 {
            return HashSet::new();
        }

        if current.open.iter().all_equal() {
            return HashSet::new();
        }

        let mut successors = HashSet::new();
        successors.insert(current.clone());
        for current_index in current.pos.keys().clone() {
            successors = successors
                .into_iter()
                .flat_map(|s| s.successors_for(current_index, self))
                .collect();
        }

        successors
            .into_iter()
            .map(|mut s| {
                s.spend_time();
                let cost = current.released_pressure - s.released_pressure;
                (s, cost)
            })
            .collect()
    }

    fn find_max_flow(&self, time: usize, count: usize) -> i64 {
        let pos = match count {
            1 => HashMap::from([(Who::Me, (name_to_int("AA"), None))]),
            2 => HashMap::from([
                (Who::Me, (name_to_int("AA"), None)),
                (Who::Elephant, (name_to_int("AA"), None)),
            ]),
            _ => todo!(),
        };
        let mut open = [false; MAX_VALVES];
        for name in 0..MAX_VALVES {
            open[name] = self.valves[name].map(|v| v.flow_rate == 0).unwrap_or(true);
        }

        let result = astar(
            &SearchState {
                pos,
                open,
                time,
                released_pressure: 0,
                trail: vec![],
            },
            |current| self.successors(current),
            |s| s.remaining(self),
            |s| s.done(self),
        );

        -result.unwrap().1
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Who {
    Me,
    Elephant,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SearchState {
    pos: HashMap<Who, (usize, Option<usize>)>,
    open: [bool; MAX_VALVES],
    time: usize,
    released_pressure: i64,
    trail: Vec<(Who, usize, usize)>,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for SearchState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.released_pressure.hash(state);
        self.time.hash(state);
        self.open.hash(state);
        self.pos.values().sorted().collect::<Vec<_>>().hash(state);
    }
}

impl SearchState {
    fn done(&self, cave: &Cave) -> bool {
        cave.valves
            .iter()
            .enumerate()
            .filter(|(index, valve)| valve.is_some())
            .all(|(index, valve)| self.open[index])
            || self.time == 0
    }

    fn remaining(&self, cave: &Cave) -> i64 {
        -1000000
    }

    fn open(&mut self, worker_index: &Who, open_index: usize, flow_rate: usize) {
        self.open[open_index] = true;
        self.pos.get_mut(worker_index).unwrap().1 = None;
        self.released_pressure += ((self.time - 1) * flow_rate) as i64;
        self.trail.push((
            worker_index.clone(),
            self.pos.get(worker_index).unwrap().0,
            self.time,
        ))
    }

    fn successors_for(mut self, worker_index: &Who, cave: &Cave) -> Vec<SearchState> {
        let current_name = self.pos.get(worker_index).unwrap().0;
        let current_valve = cave.valves[current_name].unwrap();

        let mut successors = vec![];

        for next_name in 0..MAX_VALVES {
            if !cave.neighbours[current_name][next_name] {
                continue;
            }

            if self.pos.get(worker_index).unwrap().1 == Some(next_name) {
                continue;
            }

            let mut pos = self.pos.clone();
            for who in self.pos.keys() {
                let (pos, prev) = pos.get_mut(who).unwrap();
                if *who == *worker_index {
                    *pos = next_name;
                    *prev = Some(current_name);
                }
            }

            successors.push(SearchState {
                pos,
                open: self.open.clone(),
                time: self.time,
                released_pressure: self.released_pressure,
                trail: self.trail.clone(),
            });
        }

        let successor = if current_valve.flow_rate > 0 {
            if !self.open[current_name] {
                self.open(worker_index, current_name, current_valve.flow_rate);
                Some(self)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(successor) = successor {
            successors.push(successor);
        }

        successors
    }

    fn spend_time(&mut self) {
        self.time -= 1;
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
            (
                name_to_int("AA"),
                (Valve { flow_rate: 0 }, vec![name_to_int("BB")]),
            ),
            (
                name_to_int("BB"),
                (Valve { flow_rate: 13 }, vec![name_to_int("AA")]),
            ),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364, max);
    }

    #[test]
    fn less_simple() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            (
                name_to_int("AA"),
                (Valve { flow_rate: 0 }, vec![name_to_int("BB")]),
            ),
            (
                name_to_int("BB"),
                (
                    Valve { flow_rate: 13 },
                    vec![name_to_int("AA"), name_to_int("CC")],
                ),
            ),
            (
                name_to_int("CC"),
                (Valve { flow_rate: 2 }, vec![name_to_int("BB")]),
            ),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364 + 52, max);
    }

    #[test]
    fn even_less_simple() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            (
                name_to_int("AA"),
                (Valve { flow_rate: 2 }, vec![name_to_int("BB")]),
            ),
            (
                name_to_int("BB"),
                (
                    Valve { flow_rate: 13 },
                    vec![name_to_int("AA"), name_to_int("CC")],
                ),
            ),
            (
                name_to_int("CC"),
                (Valve { flow_rate: 100 }, vec![name_to_int("BB")]),
            ),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(27 * 100 + 25 * 13 + 23 * 2, max);
    }

    #[test]
    fn still_less_simple() {
        let time_available = 10;
        let cave = Cave::new(HashMap::from([
            (
                name_to_int("AA"),
                (Valve { flow_rate: 2 }, vec![name_to_int("BB")]),
            ),
            (
                name_to_int("BB"),
                (
                    Valve { flow_rate: 13 },
                    vec![name_to_int("AA"), name_to_int("CC")],
                ),
            ),
            (
                name_to_int("CC"),
                (Valve { flow_rate: 100 }, vec![name_to_int("BB")]),
            ),
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
