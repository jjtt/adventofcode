use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
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
        let mut min = (usize::MAX, usize::MAX, usize::MAX);
        let mut max = (usize::MIN, usize::MIN, usize::MIN);
        let cubes = input
            .trim()
            .lines()
            .map(|l| scan_fmt!(l, "{d},{d},{d}", usize, usize, usize).expect("3 coords"))
            .map(|(x, y, z)| (x + 1, y + 1, z + 1))
            .map(|(x, y, z)| {
                min = (min.0.min(x), min.1.min(y), min.2.min(z));
                max = (max.0.max(x), max.1.max(y), max.2.max(z));
                (x, y, z)
            })
            .collect();
        assert!(min.0.min(min.1).min(min.2) > 0);
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
    fn count_sides(&self) -> usize {
        let mut sides = 0;
        for x in self.minx..=self.maxx {
            for y in self.miny..=self.maxy {
                for z in self.minz..=self.maxz {
                    if self.cubes.contains(&(x, y, z)) {
                        sides += if self.cubes.contains(&(x - 1, y, z)) {
                            0
                        } else {
                            1
                        };
                        sides += if self.cubes.contains(&(x + 1, y, z)) {
                            0
                        } else {
                            1
                        };
                        sides += if self.cubes.contains(&(x, y - 1, z)) {
                            0
                        } else {
                            1
                        };
                        sides += if self.cubes.contains(&(x, y + 1, z)) {
                            0
                        } else {
                            1
                        };
                        sides += if self.cubes.contains(&(x, y, z - 1)) {
                            0
                        } else {
                            1
                        };
                        sides += if self.cubes.contains(&(x, y, z + 1)) {
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

    fn holes(&self) -> Cubes {
        let mut holes = HashSet::new();

        for x in self.minx..=self.maxx {
            for y in self.miny..=self.maxy {
                for z in self.minz..=self.maxz {
                    if !self.cubes.contains(&(x, y, z)) {
                        holes.insert((x, y, z));
                    }
                }
            }
        }

        let mut outside = vec![(0, 0, 0)];
        let mut visited = HashSet::new();

        while let Some((x, y, z)) = outside.pop() {
            if visited.contains(&(x, y, z)) {
                continue;
            }
            visited.insert((x, y, z));
            holes.remove(&(x, y, z));
            if x >= self.minx
                && !visited.contains(&(x - 1, y, z))
                && !self.cubes.contains(&(x - 1, y, z))
            {
                outside.push((x - 1, y, z));
            }
            if x <= self.maxx
                && !visited.contains(&(x + 1, y, z))
                && !self.cubes.contains(&(x + 1, y, z))
            {
                outside.push((x + 1, y, z));
            }
            if y >= self.miny
                && !visited.contains(&(x, y - 1, z))
                && !self.cubes.contains(&(x, y - 1, z))
            {
                outside.push((x, y - 1, z));
            }
            if y <= self.maxy
                && !visited.contains(&(x, y + 1, z))
                && !self.cubes.contains(&(x, y + 1, z))
            {
                outside.push((x, y + 1, z));
            }
            if z >= self.minz
                && !visited.contains(&(x, y, z - 1))
                && !self.cubes.contains(&(x, y, z - 1))
            {
                outside.push((x, y, z - 1));
            }
            if z <= self.maxz
                && !visited.contains(&(x, y, z + 1))
                && !self.cubes.contains(&(x, y, z + 1))
            {
                outside.push((x, y, z + 1));
            }
        }

        Cubes {
            minx: self.minx,
            miny: self.miny,
            minz: self.minz,
            maxx: self.maxx,
            maxy: self.maxy,
            maxz: self.maxz,
            cubes: holes,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let cubes = Cubes::parse_cubes(input);
    cubes.count_sides()
}

pub fn part2(input: &str) -> usize {
    let cubes = Cubes::parse_cubes(input);
    cubes.count_sides() - cubes.holes().count_sides()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_hole() {
        let cubes = Cubes {
            minx: 1,
            miny: 1,
            minz: 1,
            maxx: 3,
            maxy: 3,
            maxz: 3,
            cubes: vec![
                (1, 1, 1),
                (1, 1, 2),
                (1, 1, 3),
                (1, 2, 1),
                (1, 2, 2),
                (1, 2, 3),
                (1, 3, 1),
                (1, 3, 2),
                (1, 3, 3),
                (2, 1, 1),
                (2, 1, 2),
                (2, 1, 3),
                (2, 2, 1),
                //(2, 2, 2),
                (2, 2, 3),
                (2, 3, 1),
                (2, 3, 2),
                (2, 3, 3),
                (3, 1, 1),
                (3, 1, 2),
                (3, 1, 3),
                (3, 2, 1),
                (3, 2, 2),
                (3, 2, 3),
                (3, 3, 1),
                (3, 3, 2),
                (3, 3, 3),
            ]
            .into_iter()
            .collect(),
        };
        let holes = cubes.holes();
        assert_eq!(1, holes.cubes.len());
        assert!(holes.cubes.contains(&(2, 2, 2)));
    }
    #[test]
    fn smaller_hole() {
        let cubes = Cubes {
            minx: 1,
            miny: 1,
            minz: 1,
            maxx: 3,
            maxy: 3,
            maxz: 3,
            cubes: vec![
                (1, 2, 2),
                (2, 1, 2),
                (2, 2, 1),
                //(2, 2, 2),
                (2, 2, 3),
                (2, 3, 2),
                (3, 2, 2),
            ]
            .into_iter()
            .collect(),
        };
        let holes = cubes.holes();
        assert_eq!(1, holes.cubes.len());
        assert!(holes.cubes.contains(&(2, 2, 2)));
    }

    #[test]
    fn holes() {
        let cubes = Cubes::parse_cubes("sample.txt");
        let holes = cubes.holes();
        assert_eq!(1, holes.cubes.len());
        assert!(holes.cubes.contains(&(2 + 1, 2 + 1, 5 + 1)));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(64, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(58, part2("sample.txt"));
    }
}
