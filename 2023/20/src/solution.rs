use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

type Targets<'a> = Vec<&'a str>;

#[derive(Debug)]
enum Module<'a> {
    Broadcaster(Targets<'a>),
    FlipFlop(bool, Targets<'a>),
    Conjunction(HashMap<&'a str, bool>, Targets<'a>, usize),
}

impl<'a> Module<'a> {
    fn parse_with_label(input: &'a str) -> (&'a str, Module<'a>) {
        let (type_and_label, targets) = input.split_once(" -> ").expect("a valid module config");
        let targets = targets.split(", ").collect::<Vec<_>>();
        match type_and_label {
            "broadcaster" => (type_and_label, Module::Broadcaster(targets)),
            _ if type_and_label.starts_with('%') => {
                (&type_and_label[1..], Module::FlipFlop(false, targets))
            }
            _ if type_and_label.starts_with('&') => (
                &type_and_label[1..],
                Module::Conjunction(HashMap::new(), targets, 0),
            ),
            _ => panic!("invalid module type: {type_and_label}"),
        }
    }

    fn outputs(&self) -> Targets<'a> {
        // todo: no clones?
        match self {
            Module::Broadcaster(targets) => targets.clone(),
            Module::FlipFlop(_, targets) => targets.clone(),
            Module::Conjunction(_, targets, _) => targets.clone(),
        }
    }

    fn apply(
        &mut self,
        signal: bool,
        caller_label: &'a str,
        module_label: &'a str,
    ) -> Vec<(bool, &'a str, &'a str)> {
        match self {
            Module::Broadcaster(targets) => targets
                .iter()
                .map(|&target| (signal, module_label, target))
                .collect(),
            Module::FlipFlop(state, targets) => {
                if !signal {
                    *state = !*state;
                    targets
                        .iter()
                        .map(|&target| (*state, module_label, target))
                        .collect()
                } else {
                    Vec::new()
                }
            }
            Module::Conjunction(states, targets, num_inputs) => {
                states.insert(caller_label, signal);
                assert!(!states.is_empty());
                assert!(*num_inputs > 0);
                let send = states.values().filter(|&state| *state).count() != *num_inputs;
                targets
                    .iter()
                    .map(|&target| (send, module_label, target))
                    .collect()
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut modules = input
        .trim()
        .lines()
        .map(Module::parse_with_label)
        .collect::<HashMap<_, _>>();

    let inputs =
        modules
            .values()
            .flat_map(Module::outputs)
            .fold(HashMap::new(), |mut map, input| {
                map.entry(input).and_modify(|e| *e += 1).or_insert(1usize);
                map
            });

    for (&label, module) in modules.iter_mut() {
        if let Module::Conjunction(_, _, num_inputs) = module {
            *num_inputs = *inputs.get(label).unwrap();
        }
    }
    modules
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();

    let mut modules = parse(&input);

    let mut highs = 0;
    let mut lows = 0;
    for _ in 0..1000 {
        lows += 1;
        let start = modules
            .get_mut("broadcaster")
            .unwrap()
            .apply(false, "button", "broadcaster");
        let mut queue = VecDeque::from(start);
        while let Some((signal, source, target)) = queue.pop_front() {
            if signal {
                highs += 1;
            } else {
                lows += 1;
            }
            if let Some(module) = modules.get_mut(target) {
                queue.extend(module.apply(signal, source, target));
            }
        }
    }

    highs * lows
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();

    let mut modules = parse(&input);

    // TODO: figure these out programmatically from the input
    let mut antepenultimate_targets = vec!["kv", "jg", "rz", "mr"];

    let mut button_presses = 1;

    for i in 0..10000 {
        let start = modules
            .get_mut("broadcaster")
            .unwrap()
            .apply(false, "button", "broadcaster");
        let mut queue = VecDeque::from(start);
        while let Some((signal, source, target)) = queue.pop_front() {
            if signal && antepenultimate_targets.contains(&source) {
                button_presses *= i + 1;
                antepenultimate_targets.retain(|&t| t != source);
            }
            if antepenultimate_targets.is_empty() {
                return button_presses;
            }

            if let Some(module) = modules.get_mut(target) {
                queue.extend(module.apply(signal, source, target));
            }
        }
    }

    todo!("didn't find the answer in 10000 iterations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(32000000, part1("sample.txt"));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(11687500, part1("sample2.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(898557000, part1("input.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(238420328103151, part2("input.txt"));
    }
}
