use multimap::MultiMap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn edges_from_input(input: &str) -> MultiMap<String, String> {
    let one_way = read_to_string(input)
        .unwrap()
        .lines()
        .filter_map(|l| l.split_once("-"))
        .map(|(from, to)| (from.to_string(), to.to_string()))
        .collect::<MultiMap<String, String>>();

    // add inverse directions
    let mut edges = one_way.clone();
    for (from, to_all) in one_way.iter_all() {
        for to in to_all {
            edges.insert(to.clone(), from.clone());
        }
    }

    edges
}

fn walk(
    current: Vec<String>,
    edges: MultiMap<String, String>,
    paths: &mut Vec<Vec<String>>,
    test: fn(&String, &Vec<String>) -> bool,
) {
    let cur = current.last().unwrap();
    if cur == "end" {
        paths.push(current)
    } else {
        for next in edges.get_vec(cur).unwrap() {
            if "start" != *next && test(next, &current) {
                let mut c = current.clone();
                c.push(next.clone());
                walk(c, edges.clone(), paths, test);
            }
        }
    }
}

fn small_caves_at_most_once(next: &String, current: &Vec<String>) -> bool {
    next.to_lowercase() != *next || !current.contains(next)
}

fn one_small_cave_at_most_twice(next: &String, current: &Vec<String>) -> bool {
    if small_caves_at_most_once(next, current) {
        true
    } else {
        current
            .iter()
            .filter_map(|cave| match cave.to_lowercase() == *cave {
                true => Some(((*cave).clone(), 1)),
                false => None,
            })
            .collect::<MultiMap<String, i32>>()
            .iter_all()
            .map(|(_, times)| times.len())
            .max()
            .unwrap()
            < 2
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(10) ; "sample1")]
    #[test_case("sample2.txt" => is eq(226) ; "sample2")]
    #[test_case("input.txt" => is eq(5576) ; "input")]
    fn part1(input: &str) -> usize {
        let edges = edges_from_input(input);

        let mut paths = vec![];

        walk(
            vec!["start".to_string()],
            edges,
            &mut paths,
            small_caves_at_most_once,
        );

        paths.len()
    }

    #[test_case("sample1.txt" => is eq(36) ; "sample1")]
    #[test_case("sample2.txt" => is eq(3509) ; "sample2")]
    #[test_case("input.txt" => is eq(152837) ; "input")]
    fn part2(input: &str) -> usize {
        let edges = edges_from_input(input);

        let mut paths = vec![];

        walk(
            vec!["start".to_string()],
            edges,
            &mut paths,
            one_small_cave_at_most_twice,
        );

        paths.len()
    }
}
