extern crate scan_fmt;

use multimap::MultiMap;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

fn parse(file: &str) -> MultiMap<String, (String, u32)> {
    let mut cities: MultiMap<String, (String, u32)> = read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| scan_fmt!(line, "{} to {} = {d}", String, String, u32).unwrap())
        .flat_map(|(city1, city2, distance)| {
            vec![
                (city1.clone(), (city2.clone(), distance)),
                (city2, (city1, distance)),
            ]
        })
        .collect();

    let first = cities
        .keys()
        .map(String::from)
        .map(|c| (c, 0))
        .collect::<Vec<(String, u32)>>();
    cities.insert_many(String::from(""), first);

    cities
}

fn main() {
    println!("{}", part1("sample1.txt"));
    println!("{}", part1("input.txt"));
    println!("{}", part2("sample1.txt"));
    println!("{}", part2("input.txt"));
}

fn part1(file: &str) -> u32 {
    let cities = parse(file);

    find_shortest_path(&String::from(""), &cities, &mut HashSet::new())
}

fn part2(file: &str) -> u32 {
    let cities = parse(file);

    find_longest_path(&String::from(""), &cities, &mut HashSet::new())
}

fn find_shortest_path<'a>(
    cur: &'a String,
    cities: &'a MultiMap<String, (String, u32)>,
    visited: &mut HashSet<&'a String>,
) -> u32 {
    if visited.len() + 1 == cities.len() {
        return 0;
    }
    let mut min = u32::MAX;
    visited.insert(cur);
    for (city, dist) in cities.get_vec(cur).unwrap() {
        if !visited.contains(city) {
            let new_min = find_shortest_path(city, cities, visited);
            if new_min < u32::MAX && new_min + dist < min {
                min = new_min + dist;
            }
        }
    }
    visited.remove(cur);
    min
}

fn find_longest_path<'a>(
    cur: &'a String,
    cities: &'a MultiMap<String, (String, u32)>,
    visited: &mut HashSet<&'a String>,
) -> u32 {
    if visited.len() + 1 == cities.len() {
        return 0;
    }
    let mut max = 0;
    visited.insert(cur);
    for (city, dist) in cities.get_vec(cur).unwrap() {
        if !visited.contains(city) {
            let new_max = find_longest_path(city, cities, visited);
            if new_max + dist > max {
                max = new_max + dist;
            }
        }
    }
    visited.remove(cur);
    max
}
