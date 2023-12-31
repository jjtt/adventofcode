use pathfinding::prelude::*;

use std::collections::HashSet;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let edges = input
        .trim()
        .lines()
        .flat_map(|line| {
            let (from, tos) = line.split_once(": ").expect("semicolon");
            tos.split(' ')
                .flat_map(move |to| vec![((from, to), 1), ((to, from), 1)])
        })
        .collect::<Vec<Edge<&str, i32>>>();

    let vertices = edges
        .iter()
        .flat_map(|((from, to), _)| vec![*from, *to])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<&str>>();

    let mut sinks = edges.iter();
    let source = sinks.next().expect("at least one vertex").0 .0;
    let mut result = None;
    while result.is_none() {
        let sink = sinks.next().expect("at least two vertices").0 .0;

        let r = edmonds_karp_dense(&vertices, &source, &sink, edges.clone());

        if r.2.len() == 3 {
            result = Some((sink, source, r));
        }
    }

    let (sink, source, (_, _, cut)) = result.expect("found a solution");

    let segment1 = dijkstra_all(&source, |v| {
        edges
            .iter()
            .filter(|((from, _to), _)| from == v)
            .filter(|((from, to), _)| {
                !cut.contains(&((*from, *to), 1)) && !cut.contains(&((*to, *from), 1))
            })
            .map(|((_, to), _)| (*to, 1))
            .collect::<Vec<_>>()
    });
    let segment2 = dijkstra_all(&sink, |v| {
        edges
            .iter()
            .filter(|((from, _to), _)| from == v)
            .filter(|((from, to), _)| {
                !cut.contains(&((*from, *to), 1)) && !cut.contains(&((*to, *from), 1))
            })
            .map(|((_, to), _)| (*to, 1))
            .collect::<Vec<_>>()
    });

    (segment1.len() + 1) * (segment2.len() + 1)
}

pub fn part2(_input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(54, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(543564, part1("input.txt"));
    }
}
