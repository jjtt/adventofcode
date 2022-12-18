use anyhow::bail;
use pathfinding::prelude::{bfs, bfs_reach};
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

struct Cave {
    valves: HashMap<String, Valve>,
}

impl Cave {
    fn successors(&self, current: &SearchState) -> Vec<SearchState> {
        //dbg!(&current);
        if current.time == 0 {
            return vec![];
        }

        let current_name = &current.pos;
        let current_valve = self
            .valves
            .get(current_name)
            .expect("Should not escape the cave");

        let mut successors = vec![];
        if let Err(index) = current.open.binary_search(current_name) {
            let mut open = current.open.clone();
            open.insert(index, current_name.clone());

            successors.push(SearchState {
                pos: current.pos.clone(),
                open,
                time: current.time - 1,
                rate: current.rate + (current.time - 1) * current_valve.flow_rate,
            })
        }
        for next_name in current_valve.tunnels.iter() {
            let next_valve = self
                .valves
                .get(next_name)
                .expect("Should not escape the cave");

            successors.push(SearchState {
                pos: next_name.clone(),
                open: current.open.clone(),
                time: current.time - 1,
                rate: current.rate,
            });
        }
        successors
    }
    fn find_max_flow(&self) -> usize {
        let result = bfs_reach(
            SearchState {
                pos: "AA".to_string(),
                open: Vec::new(),
                time: 30,
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
    tunnels: Vec<String>,
}

impl Valve {
    fn from(input: &str) -> (String, Valve) {
        dbg!(input);
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

        let tunnels = tunnels.split(", ").map(str::to_string).collect();

        (name, Valve { flow_rate, tunnels })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct SearchState {
    pos: String,
    open: Vec<String>, // remember to keep sorted!
    time: usize,
    rate: usize,
}

pub fn part1(input: &str) -> usize {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let cave = Cave { valves };

    cave.find_max_flow()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let cave = Cave {
            valves: HashMap::from([
                (
                    "AA".to_string(),
                    Valve {
                        flow_rate: 0,
                        tunnels: vec!["BB".to_string()],
                    },
                ),
                (
                    "BB".to_string(),
                    Valve {
                        flow_rate: 13,
                        tunnels: vec!["AA".to_string()],
                    },
                ),
            ]),
        };

        let max = cave.find_max_flow();
        assert_eq!(364, max);
    }

    #[test]
    fn less_simple() {
        let cave = Cave {
            valves: HashMap::from([
                (
                    "AA".to_string(),
                    Valve {
                        flow_rate: 0,
                        tunnels: vec!["BB".to_string()],
                    },
                ),
                (
                    "BB".to_string(),
                    Valve {
                        flow_rate: 13,
                        tunnels: vec!["AA".to_string(), "CC".to_string()],
                    },
                ),
                (
                    "CC".to_string(),
                    Valve {
                        flow_rate: 2,
                        tunnels: vec!["BB".to_string()],
                    },
                ),
            ]),
        };

        let max = cave.find_max_flow();
        assert_eq!(364 + 52, max);
    }

    #[test]
    fn parsing() {
        assert_eq!(
            (
                "AA".to_string(),
                Valve {
                    flow_rate: 0,
                    tunnels: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
                }
            ),
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")
        );
    }

    #[test]
    fn part1_sample() {
        assert_eq!(1651, part1("sample.txt"));
    }
}
