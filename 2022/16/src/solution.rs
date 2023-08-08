use itertools::Itertools;
use pathfinding::prelude::{astar, dijkstra_all, Matrix};
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
        let valve_names = valve_map.keys().sorted().map(String::as_str).collect();
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

    fn find_max_flow(&self, time: usize, count: usize) -> usize {
        dbg!(self);

        let result = astar(
            &SearchState::new(count, time, self),
            |s| s.successors(self),
            |s| s.remaining(self),
            |s| s.done(self),
        );

        if let Some((path, _)) = result {
            path.last().unwrap().total_flow()
        } else {
            0
        }
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

fn name_to_int(name: &str, names: &Vec<&str>) -> usize {
    names.iter().position(|n| *n == name).unwrap()
}

fn int_to_name(name: usize, names: &Vec<&str>) -> String {
    names[name].to_string()
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Worker {
    pos: usize,
    travel_time_left: usize,
    opening: bool,
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct BitField {
    bits: usize,
}

impl Debug for BitField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#020b}", self.bits)
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct SearchState {
    workers: Vec<Worker>, // keep sorted since all workers are equal
    open: BitField,
    released_pressure: usize,
    flow_rate: usize,
    time: usize,
}

impl SearchState {
    fn new(num_workers: usize, time: usize, cave: &Cave) -> SearchState {
        let workers = (0..num_workers)
            .map(|_| Worker {
                pos: 0,
                travel_time_left: 0,
                opening: false,
            })
            .collect();
        let open = cave
            .flow_rates
            .iter()
            .enumerate()
            .fold(usize::MAX, |acc, (index, flow_rate)| {
                acc & !(((*flow_rate > 0) as usize) << index)
            });
        SearchState {
            workers,
            open: BitField { bits: open },
            released_pressure: 0,
            flow_rate: 0,
            time,
        }
    }

    fn open_and_all_moves(&self, cave: &Cave, worker_index: usize) -> Vec<SearchState> {
        if self.all_open() {
            return vec![self.clone()];
        }

        let worker = &self.workers[worker_index];

        if worker.travel_time_left > 0 {
            let mut traveling = self.clone();
            traveling.workers[worker_index].travel_time_left -= 1;
            return vec![traveling];
        }

        let mut successors = vec![];

        let from = worker.pos;

        if !self.is_open(from) {
            let mut opened = self.clone();
            opened.open(from);
            opened.flow_rate += cave.flow_rates[from]; // TODO: can't update the flow rate yet?
            opened.workers[worker_index].opening = true;
            successors.push(opened);
        }

        for to in 0..cave.reachable.rows {
            let time_to_travel = cave.reachable[(from, to)];
            if !self.is_open(to) && time_to_travel > 0 && time_to_travel <= self.time {
                let mut target = self.clone();
                target.workers[worker_index].pos = to;
                target.workers[worker_index].travel_time_left = time_to_travel - 1;
                successors.push(target);
            }
        }

        successors
    }

    fn successors(&self, cave: &Cave) -> Vec<(SearchState, i64)> {
        if self.time == 0 {
            return vec![];
        }

        let mut current = self.clone();
        current.time -= 1;
        current.released_pressure += current.flow_rate;
        for worker in &mut current.workers {
            worker.opening = false;
        }
        let mut successors = vec![current];
        for worker_index in 0..self.workers.len() {
            successors = successors
                .into_iter()
                .flat_map(|s| s.open_and_all_moves(cave, worker_index))
                .collect();
        }

        successors
            .into_iter()
            .map(|s| {
                let gain = (s.flow_rate * (1 + s.time)) as i64;
                (s, -gain)
            })
            .collect()
    }

    fn remaining(&self, cave: &Cave) -> i64 {
        let mut remaining = 0;
        let mut unused_valves = usize::MAX;

        // take full flow from the valves workers are traveling to currently
        for worker in &self.workers {
            let use_valve = 1 << worker.pos;
            if !self.is_open(worker.pos) && (unused_valves & use_valve) > 0 {
                unused_valves &= !use_valve;
                remaining += cave.flow_rates[worker.pos]
                    * if worker.opening {
                        assert_eq!(worker.travel_time_left, 0);
                        1 + self.time - worker.travel_time_left
                    } else {
                        self.time - worker.travel_time_left
                    };
            }
        }

        // TODO consider other closed vales potential also?

        //dbg!((self, remaining));

        -(remaining as i64)
    }

    fn done(&self, _cave: &Cave) -> bool {
        self.all_open()
    }

    fn total_flow(&self) -> usize {
        self.released_pressure + self.time * self.flow_rate
    }

    fn open(&mut self, index: usize) {
        self.open.bits |= 1_usize << index;
    }

    fn is_open(&self, index: usize) -> bool {
        self.open.bits & (1_usize << index) > 0
    }

    fn all_open(&self) -> bool {
        self.open.bits == usize::MAX
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
        assert_eq!(0, name_to_int("AA", &vec!["AA", "AB", "AC", "AD"]));
        assert_eq!(1, name_to_int("AB", &vec!["AA", "AB", "AC", "AD"]));
        assert_eq!(2, name_to_int("AC", &vec!["AA", "AB", "AC", "AD"]));
        assert_eq!(3, name_to_int("AD", &vec!["AA", "AB", "AC", "AD"]));
    }

    #[test]
    fn index_to_name() {
        assert_eq!("AA", int_to_name(0, &vec!["AA", "AB", "AC", "AD"]));
        assert_eq!("AD", int_to_name(3, &vec!["AA", "AB", "AC", "AD"]));
    }

    #[test]
    fn name_to_index_to_name() {
        assert_eq!(
            "ZX",
            int_to_name(
                name_to_int("ZX", &vec!["AA", "AB", "AC", "ZX"]),
                &vec!["AA", "AB", "AC", "ZX"]
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
