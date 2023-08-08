use itertools::Itertools;
use pathfinding::prelude::{dfs_reach, dijkstra_all, Matrix};
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug)]
struct Cave {
    reachable: Matrix<usize>,
    flow_rates: Vec<usize>,
}

impl Cave {
    fn new(valve_map: HashMap<String, (Valve, Vec<String>)>) -> Cave {
        let valve_names: Vec<&str> = valve_map.keys().sorted().map(String::as_str).collect();
        let valve_map = valve_map
            .iter()
            .map(|(name, (valve, tunnels))| {
                (
                    name_to_int(name, &valve_names),
                    (
                        valve,
                        tunnels
                            .iter()
                            .map(|name| name_to_int(name, &valve_names))
                            .collect::<Vec<_>>(),
                    ),
                )
            })
            .collect::<Vec<_>>();

        let valve_count = valve_map.len();
        let mut neighbours = Matrix::new(valve_count, valve_count, false);
        let mut flow_rates = vec![0; valve_count];
        for (from, (valve, to)) in valve_map.iter() {
            flow_rates[*from] = valve.flow_rate;
            for to in to {
                neighbours[(*from, *to)] = true;
            }
        }

        let mut reachable = Matrix::new(valve_count, valve_count, 0);
        for (from, _) in valve_map {
            let result = dijkstra_all(&from, |from| {
                let mut successors = vec![];
                for to in 0..valve_count {
                    if neighbours[(*from, to)] {
                        successors.push((to, 1));
                    }
                }
                successors
            });
            for (to, (_, cost)) in result {
                reachable[(from, to)] = cost;
            }
        }

        Cave {
            reachable,
            flow_rates,
        }
    }

    fn init_open(&self) -> BitField {
        BitField {
            bits: self
                .flow_rates
                .iter()
                .enumerate()
                .fold(usize::MAX, |acc, (index, flow_rate)| {
                    acc & !(((*flow_rate > 0) as usize) << index)
                }),
        }
    }

    fn find_max_flow(&self, time: usize, count: usize) -> usize {
        let open = self.init_open();
        let pos_and_time: Vec<(usize, usize)> = (0..count).map(|_| (0, time)).collect();

        let result = dfs_reach(
            (0, open, pos_and_time),
            |(pressure_released, open, pos_and_time)| {
                pos_and_time
                    .iter()
                    .enumerate()
                    .cartesian_product(
                        self.flow_rates
                            .iter()
                            .enumerate()
                            .filter(|(to, _)| !open.is_open(*to)),
                    )
                    .filter(|((_, (pos, time)), (to, _))| *time > self.reachable[(*pos, *to)])
                    .map(|((who, (pos, time)), (to, rate))| {
                        let time_left = time - self.reachable[(*pos, to)] - 1;
                        let new_pressure_released = *pressure_released + time_left * rate;
                        let mut new_open = *open;
                        new_open.open(to);
                        let mut new_pos_and_time = pos_and_time.clone();
                        new_pos_and_time[who] = (to, time_left);
                        (new_pressure_released, new_open, new_pos_and_time)
                    })
                    .collect::<Vec<_>>()
            },
        );

        result.map(|(released, _, _)| released).max().unwrap()
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Valve {
    flow_rate: usize,
}

impl Valve {
    fn from(input: &str) -> (String, (Valve, Vec<String>)) {
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

        let tunnels = tunnels.split(", ").map(String::from).collect();

        (name.clone(), (Valve { flow_rate }, tunnels))
    }
}

fn name_to_int(name: &str, names: &[&str]) -> usize {
    names.iter().position(|n| *n == name).unwrap()
}

#[allow(dead_code)]
fn int_to_name(name: usize, names: &[&str]) -> String {
    names[name].to_string()
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Worker {
    pos: usize,
    travel_time_left: usize,
    opening: bool,
}

#[derive(Eq, PartialEq, Clone, Hash, Copy)]
struct BitField {
    bits: usize,
}

impl Debug for BitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#020b}", self.bits)
    }
}

impl BitField {
    fn open(&mut self, index: usize) {
        self.bits |= 1_usize << index;
    }

    fn is_open(&self, index: usize) -> bool {
        self.bits & (1_usize << index) > 0
    }
}

pub fn part1(input: &str) -> usize {
    let valves = read_to_string(input)
        .unwrap()
        .lines()
        .map(Valve::from)
        .collect();

    let time_available = 30;
    let cave = Cave::new(valves);

    cave.find_max_flow(time_available, 1)
}

pub fn part2(input: &str) -> usize {
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

    #[allow(clippy::identity_op)]
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
    fn branching() {
        let time_available = 30;
        let cave = Cave::new(HashMap::from([
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves BB, CC"),
            Valve::from("Valve BB has flow rate=2; tunnels lead to valves AA"),
            Valve::from("Valve CC has flow rate=13; tunnels lead to valves AA"),
        ]));

        let max = cave.find_max_flow(time_available, 1);
        assert_eq!(364 + 50, max);
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
                "AA".to_string(),
                (
                    Valve { flow_rate: 0 },
                    vec!["DD".to_string(), "II".to_string(), "BB".to_string(),]
                ),
            ),
            Valve::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")
        );
    }

    #[test]
    fn name_to_index() {
        assert_eq!(0, name_to_int("AA", &["AA", "AB", "AC", "AD"]));
        assert_eq!(1, name_to_int("AB", &["AA", "AB", "AC", "AD"]));
        assert_eq!(2, name_to_int("AC", &["AA", "AB", "AC", "AD"]));
        assert_eq!(3, name_to_int("AD", &["AA", "AB", "AC", "AD"]));
    }

    #[test]
    fn index_to_name() {
        assert_eq!("AA", int_to_name(0, &["AA", "AB", "AC", "AD"]));
        assert_eq!("AD", int_to_name(3, &["AA", "AB", "AC", "AD"]));
    }

    #[test]
    fn name_to_index_to_name() {
        assert_eq!(
            "ZX",
            int_to_name(
                name_to_int("ZX", &["AA", "AB", "AC", "ZX"]),
                &["AA", "AB", "AC", "ZX"]
            )
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
