#[macro_use]
extern crate scan_fmt;

use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use multimap::MultiMap;
use std::collections::HashSet;
use std::fs::read_to_string;

type Point = (i32, i32, i32);

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Eq, Hash)]
enum Rotation {
    XYZ,
    YZX,
    ZXY,
    XnYnZ,
    YnZnX,
    ZnXnY,
    XYnZn,
    YZnXn,
    ZXnYn,
    XnYZn,
    YnZXn,
    ZnXYn,
    XnZnYn,
    ZnYnXn,
    YnXnZn,
    XZYn,
    ZYXn,
    YXZn,
    XnZY,
    ZnYX,
    YnXZ,
    XZnY,
    ZYnX,
    YXnZ,
}

impl Rotation {
    fn rotate(&self, p: Point) -> Point {
        match self {
            Self::XYZ => p.clone(),
            Self::YZX => (p.1, p.2, p.0),
            Self::ZXY => (p.2, p.0, p.1),
            Self::XnYnZ => (-p.0, -p.1, p.2),
            Self::YnZnX => (-p.1, -p.2, p.0),
            Self::ZnXnY => (-p.2, -p.0, p.1),
            Self::XYnZn => (p.0, -p.1, -p.2),
            Self::YZnXn => (p.1, -p.2, -p.0),
            Self::ZXnYn => (p.2, -p.0, -p.1),
            Self::XnYZn => (-p.0, p.1, -p.2),
            Self::YnZXn => (-p.1, p.2, -p.0),
            Self::ZnXYn => (-p.2, p.0, -p.1),
            Self::XnZnYn => (-p.0, -p.2, -p.1),
            Self::ZnYnXn => (-p.2, -p.1, -p.0),
            Self::YnXnZn => (-p.1, -p.0, -p.2),
            Self::XZYn => (p.0, p.2, -p.1),
            Self::ZYXn => (p.2, p.1, -p.0),
            Self::YXZn => (p.1, p.0, -p.2),
            Self::XnZY => (-p.0, p.2, p.1),
            Self::ZnYX => (-p.2, p.1, p.0),
            Self::YnXZ => (-p.1, p.0, p.2),
            Self::XZnY => (p.0, -p.2, p.1),
            Self::ZYnX => (p.2, -p.1, p.0),
            Self::YXnZ => (p.1, -p.0, p.2),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Op {
    rotation: Rotation,
    translation: Point,
}

fn main() {
    println!("Hello, world!");
}

fn total_distance_squared(beacons: Vec<&Point>) -> i32 {
    let mut d = 0;
    let length = beacons.len();
    for i in 0..length - 1 {
        for j in (i + 1)..length {
            d += distance_squared(*beacons[i], *beacons[j])
        }
    }
    d
}

fn distance_squared(b1: Point, b2: Point) -> i32 {
    (b1.0 - b2.0).pow(2) + (b1.1 - b2.1).pow(2) + (b1.2 - b2.2).pow(2)
}

fn parse_sensors(input: &str) -> Vec<(i32, Vec<Point>)> {
    let s = read_to_string(input).unwrap();

    s.split("\n\n").map(parse_sensor).collect()
}

fn parse_sensor(input: &str) -> (i32, Vec<Point>) {
    let (s, c) = input.split_once("\n").unwrap();

    (
        scan_fmt!(s, "--- scanner {d} ---", i32).unwrap(),
        parse_coordinates(c),
    )
}

fn parse_coordinates(beacons: &str) -> Vec<Point> {
    beacons
        .lines()
        .filter_map(|l| l.split(",").map(|c| c.parse().unwrap()).collect_tuple())
        .collect()
}

fn triplet_distances(coords: &Vec<Point>) -> MultiMap<i32, [&Point; 3]> {
    let mut dists = MultiMap::new();

    for nlet in coords.iter().combinations(3) {
        if !collinear(*nlet[0], *nlet[1], *nlet[2]) {
            let clone = [nlet[0], nlet[1], nlet[2]];
            let dist = total_distance_squared(nlet);
            dists.insert(dist, clone);
        }
    }

    dists
}

fn collinear(p1: Point, p2: Point, p3: Point) -> bool {
    let u = (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2);
    let v = (p1.0 - p3.0, p1.1 - p3.1, p1.2 - p3.2);
    assert!(u.0 != 0 || u.1 != 0 || u.2 != 0);
    assert!(v.0 != 0 || v.1 != 0 || v.2 != 0);

    let x = u.1 * v.2 - u.2 * v.1;
    let y = u.2 * v.0 - u.0 * v.2;
    let z = u.0 * v.1 - u.1 * v.0;

    x == 0 && y == 0 && z == 0
}

fn apply_rotation(triplet1: [&Point; 3], rotation: &Rotation) -> [Point; 3] {
    [
        rotation.rotate(*triplet1[0]),
        rotation.rotate(*triplet1[1]),
        rotation.rotate(*triplet1[2]),
    ]
}

fn apply_translation(triplet1: [&Point; 3], translation: &Point) -> [Point; 3] {
    [
        (
            triplet1[0].0 + translation.0,
            triplet1[0].1 + translation.1,
            triplet1[0].2 + translation.2,
        ),
        (
            triplet1[1].0 + translation.0,
            triplet1[1].1 + translation.1,
            triplet1[1].2 + translation.2,
        ),
        (
            triplet1[2].0 + translation.0,
            triplet1[2].1 + translation.1,
            triplet1[2].2 + translation.2,
        ),
    ]
}

fn find_op(triplet0: [&Point; 3], triplet1: [&Point; 3]) -> Option<Op> {
    for rotation in Rotation::into_enum_iter() {
        for permutation in (0..3).permutations(3) {
            let candidate = [
                triplet1[permutation[0]],
                triplet1[permutation[1]],
                triplet1[permutation[2]],
            ];
            let rotated = apply_rotation(candidate, &rotation);
            let translation = diff(triplet0[0], &rotated[0]);
            if translation == diff(triplet0[1], &rotated[1])
                && translation == diff(triplet0[2], &rotated[2])
            {
                return Some(Op {
                    rotation,
                    translation,
                });
            }
        }
    }
    None
}

fn diff(p1: &Point, p2: &Point) -> Point {
    (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2)
}

fn find_common_12(
    first: &Vec<Point>,
    second: &Vec<Point>,
) -> Option<(HashSet<Point>, HashSet<Point>, Op)> {
    let first_dists = triplet_distances(first);
    let second_dists = triplet_distances(second);

    let mut first_common = HashSet::new();
    let mut second_common = HashSet::new();

    let mut ops = HashSet::new();

    for dist in first_dists.keys() {
        if second_dists.contains_key(dist) {
            let from_first = first_dists.get_vec(dist).unwrap();
            let from_second = second_dists.get_vec(dist).unwrap();

            // TODO: there may be multiple matches, but we're only handling the first
            //assert_eq!(1, from_first.len());
            //assert_eq!(1, from_second.len());

            let from_first = &from_first[0];
            let from_second = &from_second[0];

            let op = find_op(*from_first, *from_second);
            if op.is_none() {
                //dbg!(dist);
                //dbg!(from_first);
                //dbg!(from_second);
                dbg!(&op);
            } else {
                ops.insert(op.unwrap());
                first_common.extend(from_first.iter().map(|(x, y, z)| (*x, *y, *z)));
                second_common.extend(from_second.iter().map(|(x, y, z)| (*x, *y, *z)));
            }
        }
    }

    if ops.len() != 1 {
        None
    } else {
        let op = ops.into_iter().next().unwrap();

        Some((first_common, second_common, op))
    }
}

fn combine_sensors(sensors: &Vec<(i32, Vec<Point>)>) -> (Vec<Point>, Vec<Point>) {
    let mut beacons: Vec<Point> = Vec::new();
    let mut sensor_positions = Vec::new();

    beacons.extend(&sensors[0].1);
    sensor_positions.push((0, 0, 0));

    let other_sensors = &sensors[1..];
    let mut processed_sensors = HashSet::new();

    while processed_sensors.len() < other_sensors.len() {
        for sensor in other_sensors {
            if !processed_sensors.contains(sensor) {
                match find_common_12(&beacons, &sensor.1) {
                    Some((_common0, common1, op)) => {
                        for beacon in &sensor.1 {
                            if !common1.contains(beacon) {
                                let rotated = op.rotation.rotate(*beacon);
                                let translated = (
                                    rotated.0 + op.translation.0,
                                    rotated.1 + op.translation.1,
                                    rotated.2 + op.translation.2,
                                );
                                beacons.push(translated);
                                sensor_positions.push(op.translation);
                            }
                        }

                        processed_sensors.insert(sensor);
                        break;
                    }
                    _ => (),
                }
            }
        }
    }
    (beacons, sensor_positions)
}

fn max_manhattan_distance(points: Vec<Point>) -> i32 {
    let mut max = 0;
    for pair in points.iter().combinations(2) {
        let first = pair[0];
        let second = pair[1];
        let dist =
            (first.0 - second.0).abs() + (first.1 - second.1).abs() + (first.2 - second.2).abs();
        if dist > max {
            max = dist;
        }
    }
    max
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

        let distance1 = total_distance_squared(b1.iter().collect());
        let distance2 = total_distance_squared(b2.iter().collect());
        assert_eq!(distance1, distance2);
    }

    #[test]
    fn some_triplet_distances() {
        let coords = parse_coordinates(indoc! {"
            0,0,0
            1,1,2
            2,1,2
            4,4,4
        "});

        let mut dists = triplet_distances(&coords);
        assert_eq!(1, dists.remove(&40).unwrap().len());
        assert_eq!(1, dists.remove(&76).unwrap().len());
        assert_eq!(1, dists.remove(&74).unwrap().len());
        assert_eq!(1, dists.remove(&16).unwrap().len());
        assert!(dists.is_empty());
    }

    #[test_case(vec![(0,0,0),(1,1,1)] => is eq(3); "1")]
    #[test_case(vec![(0,0,0),(1,1,1),(0,1,0)] => is eq(3); "2")]
    #[test_case(vec![(0,0,0),(1,1,1),(-1,-2,1)] => is eq(5); "3")]
    #[test_case(vec![(-1,-2,1),(0,0,0)] => is eq(4); "4")]
    fn finding_max_manhattan_distances(points: Vec<Point>) -> i32 {
        max_manhattan_distance(points)
    }

    #[test]
    fn find_common_12_from_self() {
        let beacons = parse_coordinates(indoc! {"
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

        let (common0, common1, _) = find_common_12(&beacons.clone(), &beacons).unwrap();

        assert_eq!(
            HashSet::<_>::from_iter(beacons.clone()),
            HashSet::from_iter(common0)
        );
        assert_eq!(
            HashSet::<_>::from_iter(beacons),
            HashSet::from_iter(common1)
        );
    }

    #[test]
    fn are_collinear() {
        let l1 = (1, 1, 1);
        let l2 = (2, 2, 2);
        let l3 = (3, 3, 3);
        let n = (1, 2, 3);

        assert!(collinear(l1, l2, l3));
        assert!(!collinear(l1, l2, n));
        assert!(!collinear(l1, n, l3));
        assert!(!collinear(n, l2, l3));
    }

    #[test]
    fn find_matching_ops_just_rotate() {
        let reference1 = [&(0, 0, 0), &(1, 0, 0), &(0, 1, 0)];
        let reference2 = [&(0, 0, 1), &(2, 0, 0), &(0, 3, 0)];
        let other1 = [&(0, 0, 0), &(-1, 0, 0), &(0, 1, 0)];
        let other2 = [&(0, 0, -1), &(0, -2, 0), &(-3, 0, 0)];

        assert_eq!(
            reference1,
            apply_rotation(other1, &find_op(reference1, other1).unwrap().rotation)
                .iter()
                .collect::<Vec<&Point>>()[0..3]
        );
        assert_eq!(
            reference1,
            apply_rotation(
                reference1,
                &find_op(reference1, reference1).unwrap().rotation
            )
            .iter()
            .collect::<Vec<&Point>>()[0..3]
        );
        assert_eq!(
            reference2,
            apply_rotation(other2, &find_op(reference2, other2).unwrap().rotation)
                .iter()
                .collect::<Vec<&Point>>()[0..3]
        );
    }

    #[test]
    fn find_matching_ops_rotate_permute() {
        let triplet0 = [&(0, 0, 0), &(1, 0, 0), &(0, 1, 0)];
        let triplet1 = [&(-1, 0, 0), &(0, 0, 0), &(0, 1, 0)];

        let rotated = apply_rotation(triplet1, &find_op(triplet0, triplet1).unwrap().rotation);
        assert_eq!(triplet0[0], &rotated[1]);
        assert_eq!(triplet0[1], &rotated[0]);
        assert_eq!(triplet0[2], &rotated[2]);
    }

    #[test]
    fn find_matching_ops_rotate_permute_translate() {
        let triplet0 = [&(0, 0, 0), &(1, 0, 0), &(0, 1, 0)];
        let triplet1 = [&(-1, 0, 1), &(0, 0, 1), &(0, 1, 1)];

        let op = find_op(triplet0, triplet1).unwrap();
        assert_eq!(op.translation, (0, 0, 1));
        let rotated = apply_rotation(triplet1, &op.rotation);
        let translated = apply_translation(
            rotated.iter().collect::<Vec<&Point>>()[0..3]
                .try_into()
                .expect("slice with incorrect length"),
            &op.translation,
        );
        assert_eq!(*triplet0[0], translated[1]);
        assert_eq!(triplet0[1], &translated[0]);
        assert_eq!(*triplet0[2], translated[2]);
    }

    #[test]
    fn find_common_12_from_sample_0_1() {
        let sensors = parse_sensors("sample1.txt");

        let common_from_0 = parse_coordinates(indoc! {"
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

        let (common0, common1, _) = find_common_12(&sensors[0].1, &sensors[1].1).unwrap();

        assert_eq!(common_from_0.len(), common0.len());
        assert_eq!(common_from_0.len(), common1.len());

        assert_eq!(HashSet::<_>::from_iter(common_from_0), common0);
    }

    #[test_case("sample1.txt" => is eq(79); "sample1")]
    #[test_case("input.txt" => is eq(381); "input")]
    fn part1(input: &str) -> usize {
        let sensors = parse_sensors(input);

        let (beacons, _) = combine_sensors(&sensors);

        beacons.len()
    }

    #[test_case("sample1.txt" => is eq(3621); "sample1")]
    #[test_case("input.txt" => is eq(12201); "input")]
    fn part2(input: &str) -> i32 {
        let sensors = parse_sensors(input);

        let (_, sensor_positions) = combine_sensors(&sensors);

        max_manhattan_distance(sensor_positions)
    }
}
