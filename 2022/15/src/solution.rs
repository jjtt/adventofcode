use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

type Pos = (i32, i32);

pub fn part1(input: &str) -> i32 {
    part1_for_y(input, 2000000)
}

pub fn part1_for_y(input: &str, y: i32) -> i32 {
    let sensors = parse_sensors(input);

    let known_positions = sensors
        .iter()
        .flat_map(|(s, b)| vec![*s, *b])
        .collect::<HashSet<_>>();

    sensors
        .into_iter()
        .flat_map(|s| covered_on_y(s, y))
        .filter(|p| !known_positions.contains(p))
        .collect::<HashSet<_>>()
        .len() as i32
}

pub fn part2(input: &str) -> i32 {
    part2_for_grid(input, 4000000, 4000000)
}

pub fn part2_for_grid(input: &str, maxx: i32, maxy: i32) -> i32 {
    let sensors = parse_sensors(input);

    let covered: HashSet<Pos> = sensors.into_iter().flat_map(covered).collect();

    for y in (0..=maxy).rev() {
        for x in (0..=maxx).rev() {
            if !covered.contains(&(x, y)) {
                return x * 4000000 + y;
            }
        }
    }

    panic!("Not found")
}

fn covered_on_y(sensor: (Pos, Pos), y: i32) -> HashSet<Pos> {
    let dist = manhattan(sensor.0, sensor.1);

    let offset = 0.max((sensor.0 .1 - y).abs());

    let dist_on_row = dist - offset;

    (sensor.0 .0 - dist_on_row..=sensor.0 .0 + dist_on_row)
        .map(|x| (x, y))
        .collect()
}

fn covered(sensor: (Pos, Pos)) -> HashSet<Pos> {
    dbg!(&sensor);
    let dist = manhattan(sensor.0, sensor.1);

    let mut covered = HashSet::new();
    for y in 0.max(sensor.0 .1 - dist)..=4000000.min(sensor.0 .1 + dist) {
        let offset = 0.max((sensor.0 .1 - y).abs());
        let dist_on_row = dist - offset;
        covered.extend(
            (0.max(sensor.0 .0 - dist_on_row)..=4000000.min(sensor.0 .0 + dist_on_row))
                .map(|x| (x, y)),
        );
    }
    covered
}

fn manhattan(first: Pos, second: Pos) -> i32 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

fn parse(input: &str) -> (Pos, Pos) {
    let (sx, sy, bx, by) = scan_fmt!(
        input,
        "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();
    ((sx, sy), (bx, by))
}

fn parse_sensors(input: &str) -> Vec<(Pos, Pos)> {
    read_to_string(input).unwrap().lines().map(parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn covering() {
        let covered = covered_on_y(((8, 7), (2, 10)), 10);
        assert_eq!(13, covered.len());
        assert!(covered.iter().all(|pos| vec![
            (2, 10),
            (3, 10),
            (4, 10),
            (5, 10),
            (6, 10),
            (7, 10),
            (8, 10),
            (9, 10),
            (10, 10),
            (11, 10),
            (12, 10),
            (13, 10),
            (14, 10),
        ]
        .contains(pos)))
    }

    #[test]
    fn covering_all() {
        let covered = covered(((8, 7), (8, 8)));
        assert_eq!(5, covered.len());
        let mut sorted = covered.into_iter().collect::<Vec<_>>();
        sorted.sort();
        assert_eq!(vec![(7, 7), (8, 6), (8, 7), (8, 8), (9, 7)], sorted);
    }

    #[test]
    fn parsing() {
        assert_eq!(
            ((2, 18), (-2, 15)),
            parse("Sensor at x=2, y=18: closest beacon is at x=-2, y=15")
        );
    }

    #[test]
    fn part1_sample() {
        assert_eq!(26, part1_for_y("sample.txt", 10));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(56000011, part2_for_grid("sample.txt", 20, 20));
    }
}
