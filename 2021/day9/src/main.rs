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
    map: HashMap<Point, i32>,
}

impl Map {
    fn find_low_points(&self) -> Vec<Point> {
        let mut low_points = vec![];

        for (point, value) in &self.map {
            if self.is_low_point(point) {
                low_points.push((*point).clone());
            }
        }

        low_points
    }

    fn is_low_point(&self, point: &Point) -> bool {
        let v = self.value(point).unwrap();
        let n = self.value(&point.n()).unwrap_or(&i32::MAX);
        let s = self.value(&point.s()).unwrap_or(&i32::MAX);
        let w = self.value(&point.w()).unwrap_or(&i32::MAX);
        let e = self.value(&point.e()).unwrap_or(&i32::MAX);
        v < n && v < s && v < w && v < e
    }

    fn value(&self, point: &Point) -> Option<&i32> {
        self.map.get(point)
    }
}

fn main() {
    println!("Hello, world!");

    let m = Map {
        map: [(Point { x: 0, y: 0 }, 1)].into_iter().collect(),
    };

    assert_eq!(1, *m.value(&Point { x: 0, y: 0 }).unwrap());

    assert!(m.value(&Point { x: 1, y: 0 }).is_none());
    assert!(m.value(&Point { x: 1, y: 1 }).is_none());
    assert!(m.value(&Point { x: 0, y: 1 }).is_none());
}

fn map_from_input(input: String) -> Map {
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
                        c.to_string().parse::<i32>().unwrap(),
                    )
                })
            })
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(15) ; "sample")]
    #[test_case("input.txt" => is eq(522) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let map = map_from_input(input);

        let low_points = map.find_low_points();

        low_points.iter().map(|p| map.value(p).unwrap() + 1).sum()
    }
}
