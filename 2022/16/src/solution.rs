use itertools::Itertools;
use pathfinding::prelude::bfs_reach;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

struct Cave {
    valves: HashMap<usize, Valve>,
}

impl Cave {
    fn successors(&self, current: &SearchState) -> impl IntoIterator<Item = SearchState> {
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
                s
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
        let result = bfs_reach(
            SearchState {
                pos,
                open: Vec::new(),
                time,
                rate: 0,
            },
            |current| self.successors(current),
        );

        result.map(|s| s.rate).max().unwrap()
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
    rate: usize,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for SearchState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rate.hash(state);
        self.time.hash(state);
        self.open.hash(state);
        self.pos.values().sorted().collect::<Vec<_>>().hash(state);
    }
}

impl SearchState {
    fn open(&mut self, worker_index: &Who, open_index: usize, flow_rate: usize) {
        self.open
            .insert(open_index, self.pos.get(worker_index).unwrap().0);
        self.pos.get_mut(worker_index).unwrap().1 = None;
        self.rate += (self.time - 1) * flow_rate;
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
                rate: self.rate,
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

    let cave = Cave { valves };

    cave.find_max_flow(30, 1)
}

pub fn part2(input: &str) -> usize {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let cave = Cave { valves };

    cave.find_max_flow(26, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let cave = Cave {
            valves: HashMap::from([
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
        };

        let max = cave.find_max_flow(30, 1);
        assert_eq!(364, max);
    }

    #[test]
    fn less_simple() {
        let cave = Cave {
            valves: HashMap::from([
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
        };

        let max = cave.find_max_flow(30, 1);
        assert_eq!(364 + 52, max);
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
