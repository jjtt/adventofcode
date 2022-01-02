#[macro_use]
extern crate scan_fmt;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cached]
fn total_distance_squared(beacons: Vec<(i32, i32, i32)>) -> i32 {
    let mut d = 0;
    let length = beacons.len();
    for i in 0..length - 1 {
        for j in (i + 1)..length {
            d += distance_squared(beacons[i], beacons[j])
        }
    }
    d
}

fn distance_squared(b1: (i32, i32, i32), b2: (i32, i32, i32)) -> i32 {
    (b1.0 - b2.0).pow(2) + (b1.1 - b2.1).pow(2) + (b1.2 - b2.2).pow(2)
}

fn parse_sensors(input: &str) -> Vec<(i32, Vec<(i32, i32, i32)>)> {
    let s = read_to_string(input).unwrap();

    s.split("\n\n").map(parse_sensor).collect()
}

fn parse_sensor(input: &str) -> (i32, Vec<(i32, i32, i32)>) {
    let (s, c) = input.split_once("\n").unwrap();

    (
        scan_fmt!(s, "--- scanner {d} ---", i32).unwrap(),
        parse_coordinates(c),
    )
}

fn parse_coordinates(beacons: &str) -> Vec<(i32, i32, i32)> {
    beacons
        .lines()
        .filter_map(|l| l.split(",").map(|c| c.parse().unwrap()).collect_tuple())
        .collect()
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use test_case::test_case;

    use super::*;

    #[test]
    fn total_distance_for_a_match() {
        let beacons1 = indoc! {"
            -618,-824,-621
            -537,-823,-458
            -447,-329,318
            404,-588,-901
            544,-627,-890
            528,-643,409
            -661,-816,-575
            390,-675,-793
            423,-701,434
            -345,-311,381
            459,-707,401
            -485,-357,347
        "};
        let beacons2 = indoc! {"
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            -476,619,847
            -460,603,-452
            729,430,532
            -322,571,750
            -355,545,-477
            413,935,-424
            -391,539,-444
            553,889,-390
        "};

        let b1 = parse_coordinates(beacons1);
        let b2 = parse_coordinates(beacons2);

        let distance1 = total_distance_squared(b1);
        let distance2 = total_distance_squared(b2);
        assert_eq!(distance1, distance2);

        dbg!(distance1);
    }

    #[test]
    fn find_common_12_from_sample_0_1() {
        let sensors = parse_sensors("sample1.txt");

        let commont_from_0 = parse_coordinates(indoc! {"
            -618,-824,-621
            -537,-823,-458
            -447,-329,318
            404,-588,-901
            544,-627,-890
            528,-643,409
            -661,-816,-575
            390,-675,-793
            423,-701,434
            -345,-311,381
            459,-707,401
            -485,-357,347
        "});

        let (common0, common1) = find_common_12(&sensors[0].1, &sensors[1].1).unwrap();
    }

    fn find_common_12(
        first: &Vec<(i32, i32, i32)>,
        second: &Vec<(i32, i32, i32)>,
    ) -> Option<(Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>)> {
        todo!();
        for f in first.iter().combinations(12) {
            let fcopy: Vec<(i32, i32, i32)> = f.iter().map(|c| *c.clone()).collect();
            let first_dist = total_distance_squared(fcopy.clone());
            for s in second.iter().combinations(12) {
                let scopy: Vec<(i32, i32, i32)> = s.iter().map(|c| *c.clone()).collect();
                let second_dist = total_distance_squared(scopy.clone());

                if first_dist == second_dist {
                    return Some((fcopy, scopy));
                }
            }
        }

        None
    }

    #[test_case("sample1.txt" => is eq(79); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> i32 {
        let sensors = parse_sensors(input);

        0
    }
}
