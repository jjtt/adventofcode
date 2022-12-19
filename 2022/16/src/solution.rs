use itertools::Itertools;
use pathfinding::prelude::{astar, bfs_reach};
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Cave {
    valves: HashMap<usize, Valve>,
    max_releasable_pressure: usize,
}

impl Cave {
    fn new(valves: HashMap<usize, Valve>, time_available: usize) -> Cave {
        let max_pressure = valves.values().map(|v| v.flow_rate * time_available).sum();
        Cave {
            valves,
            max_releasable_pressure: max_pressure,
        }
    }

    fn successors(&self, current: &SearchState) -> impl IntoIterator<Item = (SearchState, usize)> {
        if current.time == 0 {
            return HashSet::new();
        }

        if current.open.len() == self.valves.len() {
            return HashSet::new();
        }

        let mut successors = HashSet::new();
        successors.insert(current.clone());
        for current_index in current.pos.keys().clone() {
            successors = successors
                .into_iter()
                .flat_map(|s| s.successors_for(current_index, &self.valves))
                .collect();
        }

        successors
            .into_iter()
            .map(|mut s| {
                s.spend_time();
                let cost = self.max_releasable_pressure - s.released_pressure;
                (s, cost)
            })
            .collect()
    }

    fn find_max_flow(&self, time: usize, count: usize) -> usize {
        let pos = match count {
            1 => HashMap::from([(Who::Me, (name_to_int("AA"), None))]),
            2 => HashMap::from([
                (Who::Me, (name_to_int("AA"), None)),
                (Who::Elephant, (name_to_int("AA"), None)),
            ]),
            _ => todo!(),
        };
        let result = astar(
            &SearchState {
                pos,
                open: self
                    .valves
                    .iter()
                    .filter(|(_, v)| v.flow_rate == 0)
                    .map(|(n, _)| *n)
                    .collect(),
                time,
                released_pressure: 0,
                trail: vec![],
            },
            |current| self.successors(current),
            |s| s.remaining(self),
            |s| s.done(self),
        );

        dbg!(&result);
        dbg!(self);
        result.unwrap().0.last().unwrap().released_pressure
    }
}

#[derive(PartialEq, Debug)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<usize>,
}

impl Valve {
    fn from(input: &str) -> (usize, Valve) {
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

        (name_to_int(&name), Valve { flow_rate, tunnels })
    }
}

fn name_to_int(name: &str) -> usize {
    name.char_indices()
        .map(|(i, c)| (c as usize).pow(i as u32 + 1))
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Who {
    Me,
    Elephant,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct SearchState {
    pos: HashMap<Who, (usize, Option<usize>)>,
    open: Vec<usize>, // remember to keep sorted!
    time: usize,
    released_pressure: usize,
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
        cave.valves.keys().all(|v| self.open.contains(v)) || self.time == 0
    }

    fn remaining(&self, cave: &Cave) -> usize {
        //dbg!(cave);
        //dbg!(self);
        cave.max_releasable_pressure
            - self.released_pressure
            - cave
                .valves
                .iter()
                .filter(|(n, _)| !self.open.contains(n))
                .map(|(_, v)| v.flow_rate)
                .sum::<usize>()
    }

    fn open(&mut self, worker_index: &Who, open_index: usize, flow_rate: usize) {
        self.open
            .insert(open_index, self.pos.get(worker_index).unwrap().0);
        self.pos.get_mut(worker_index).unwrap().1 = None;
        self.released_pressure += (self.time - 1) * flow_rate;
        self.trail.push((
            worker_index.clone(),
            self.pos.get(worker_index).unwrap().0,
            self.time,
        ))
    }

    fn successors_for(
        mut self,
        worker_index: &Who,
        valves: &HashMap<usize, Valve>,
    ) -> Vec<SearchState> {
        let current_name = &self.pos.get(worker_index).unwrap().0;
        let current_valve = valves
            .get(current_name)
            .expect("Should not escape the cave");

        let mut successors = vec![];

        for next_name in current_valve.tunnels.iter() {
            if self.pos.get(worker_index).unwrap().1 == Some(*next_name) {
                continue;
            }

            let mut pos = self.pos.clone();
            for who in self.pos.keys() {
                let (pos, prev) = pos.get_mut(who).unwrap();
                if *who == *worker_index {
                    *pos = *next_name;
                    *prev = Some(*current_name);
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
            if let Err(open_index) = self.open.binary_search(current_name) {
                self.open(worker_index, open_index, current_valve.flow_rate);
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

pub fn part1(input: &str) -> usize {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let time_available = 30;
    let cave = Cave::new(valves, time_available);

    cave.find_max_flow(time_available, 1)
}

pub fn part2(input: &str) -> usize {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let time_available = 26;
    let cave = Cave::new(valves, time_available);

    cave.find_max_flow(time_available, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let time_available = 30;
        let cave = Cave::new(
            HashMap::from([
                (
                    name_to_int("AA"),
                    Valve {
                        flow_rate: 0,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
                (
                    name_to_int("BB"),
                    Valve {
                        flow_rate: 13,
                        tunnels: vec![name_to_int("AA")],
                    },
                ),
            ]),
            time_available,
        );

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364, max);
    }

    #[test]
    fn less_simple() {
        let time_available = 30;
        let cave = Cave::new(
            HashMap::from([
                (
                    name_to_int("AA"),
                    Valve {
                        flow_rate: 0,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
                (
                    name_to_int("BB"),
                    Valve {
                        flow_rate: 13,
                        tunnels: vec![name_to_int("AA"), name_to_int("CC")],
                    },
                ),
                (
                    name_to_int("CC"),
                    Valve {
                        flow_rate: 2,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
            ]),
            time_available,
        );

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364 + 52, max);
    }

    #[test]
    fn even_less_simple() {
        let time_available = 30;
        let cave = Cave::new(
            HashMap::from([
                (
                    name_to_int("AA"),
                    Valve {
                        flow_rate: 2,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
                (
                    name_to_int("BB"),
                    Valve {
                        flow_rate: 13,
                        tunnels: vec![name_to_int("AA"), name_to_int("CC")],
                    },
                ),
                (
                    name_to_int("CC"),
                    Valve {
                        flow_rate: 100,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
            ]),
            time_available,
        );

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(27 * 100 + 25 * 13 + 23 * 2, max);
    }

    #[test]
    fn still_less_simple() {
        let time_available = 10;
        let cave = Cave::new(
            HashMap::from([
                (
                    name_to_int("AA"),
                    Valve {
                        flow_rate: 2,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
                (
                    name_to_int("BB"),
                    Valve {
                        flow_rate: 13,
                        tunnels: vec![name_to_int("AA"), name_to_int("CC")],
                    },
                ),
                (
                    name_to_int("CC"),
                    Valve {
                        flow_rate: 100,
                        tunnels: vec![name_to_int("BB")],
                    },
                ),
            ]),
            time_available,
        );

        let max = cave.find_max_flow(time_available, 2);
        assert_eq!(7 * 100 + 8 * 13 + 6 * 2, max);
    }

    #[test]
    fn parsing() {
        assert_eq!(
            (
                name_to_int("AA"),
                Valve {
                    flow_rate: 0,
                    tunnels: vec![name_to_int("DD"), name_to_int("II"), name_to_int("BB")],
                }
            ),
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")
        );
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
