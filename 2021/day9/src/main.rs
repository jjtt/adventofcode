use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn n(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn s(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn w(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn e(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<Point, u32>,
}

impl Map {
    fn new(input: String) -> Map {
        Map {
            map: input
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().map(move |(x, c)| {
                        (
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            c.to_digit(10).unwrap(),
                        )
                    })
                })
                .collect(),
        }
    }

    fn find_low_points(&self) -> Vec<Point> {
        let mut low_points = vec![];

        for point in self.map.keys() {
            if self.is_low_point(point) {
                low_points.push((*point).clone());
            }
        }

        low_points
    }

    fn is_low_point(&self, point: &Point) -> bool {
        let v = self.value(point).unwrap();
        let n = self.value(&point.n()).unwrap_or(&u32::MAX);
        let s = self.value(&point.s()).unwrap_or(&u32::MAX);
        let w = self.value(&point.w()).unwrap_or(&u32::MAX);
        let e = self.value(&point.e()).unwrap_or(&u32::MAX);
        v < n && v < s && v < w && v < e
    }

    fn value(&self, point: &Point) -> Option<&u32> {
        self.map.get(point)
    }

    fn basin_size(&self, point: &Point) -> i32 {
        let mut basin = vec![];
        self.basin(point, &mut basin);
        basin.len() as i32
    }

    fn basin(&self, point: &Point, basin: &mut Vec<Point>) {
        let value = self.value(point).unwrap_or(&u32::MAX);
        if !basin.contains(&point) && *value < 9 {
            basin.push(point.clone());
            self.basin(&point.n(), basin);
            self.basin(&point.s(), basin);
            self.basin(&point.w(), basin);
            self.basin(&point.e(), basin);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(15) ; "sample")]
    #[test_case("input.txt" => is eq(522) ; "input")]
    fn part1(input: &str) -> u32 {
        let input = read_to_string(input).unwrap();

        let map = Map::new(input);

        map.find_low_points()
            .iter()
            .map(|p| map.value(p).unwrap() + 1)
            .sum()
    }

    #[test_case("sample1.txt" => is eq(1134) ; "sample")]
    #[test_case("input.txt" => is eq(916688) ; "input")]
    fn part2(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let map = Map::new(input);

        let mut basin_sizes: Vec<i32> = map
            .find_low_points()
            .iter()
            .map(|low| map.basin_size(low))
            .collect();

        basin_sizes.sort();
        basin_sizes.reverse();

        basin_sizes[0..3].iter().product()
    }
}
