use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

struct Cubes {
    minx: usize,
    miny: usize,
    minz: usize,
    maxx: usize,
    maxy: usize,
    maxz: usize,
    cubes: HashSet<Cube>,
}

type Cube = (usize, usize, usize);

impl Cubes {
    fn parse_cubes(input: &str) -> Cubes {
        let input = read_to_string(input).unwrap();
        let mut min = (0, 0, 0); // should be usize::MAX?
        let mut max = (0, 0, 0);
        let cubes = input
            .trim()
            .lines()
            .map(|l| scan_fmt!(l, "{d},{d},{d}", usize, usize, usize).expect("3 coords"))
            .map(|(x, y, z)| {
                min = (min.0.min(x), min.1.min(y), min.2.min(z));
                max = (max.0.max(x), max.1.max(y), max.2.max(z));
                (x, y, z)
            })
            .collect();
        Cubes {
            minx: min.0,
            miny: min.1,
            minz: min.2,
            maxx: max.0,
            maxy: max.1,
            maxz: max.2,
            cubes,
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let cubes = Cubes::parse_cubes(input);

    let mut sides = 0;

    for x in cubes.minx..=cubes.maxx {
        for y in cubes.miny..=cubes.maxy {
            for z in cubes.minz..=cubes.maxz {
                if cubes.cubes.contains(&(x, y, z)) {
                    sides += if cubes.cubes.contains(&(x - 1, y, z)) {
                        0
                    } else {
                        1
                    };
                    sides += if cubes.cubes.contains(&(x + 1, y, z)) {
                        0
                    } else {
                        1
                    };
                    sides += if cubes.cubes.contains(&(x, y - 1, z)) {
                        0
                    } else {
                        1
                    };
                    sides += if cubes.cubes.contains(&(x, y + 1, z)) {
                        0
                    } else {
                        1
                    };
                    sides += if cubes.cubes.contains(&(x, y, z - 1)) {
                        0
                    } else {
                        1
                    };
                    sides += if cubes.cubes.contains(&(x, y, z + 1)) {
                        0
                    } else {
                        1
                    };
                }
            }
        }
    }

    sides
}

pub fn part2(input: &str) -> i32 {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(64, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(58, part2("sample.txt"));
    }
}
