#![feature(slice_group_by)]
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn new(line: &str) -> Line {
        let coords = line
            .split(" -> ")
            .map(|l| {
                l.split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        Line {
            x1: *&coords[0][0],
            y1: *&coords[0][1],
            x2: *&coords[1][0],
            y2: *&coords[1][1],
        }
    }

    fn is_hori_vert(&self) -> bool {
        self.is_hori() || self.is_vert()
    }

    fn is_hori(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vert(&self) -> bool {
        self.x1 == self.x2
    }

    fn points(&self) -> Vec<(i32, i32)> {
        let k = (self.y2 - self.y1) as f32 / (self.x2 - self.x1) as f32;
        let k_inv = (self.x2 - self.x1) as f32 / (self.y2 - self.y1) as f32;
        if self.is_vert() || k > 1.0 {
            let points = Line::points_on_line(self.y1, self.x1, self.y2, k_inv);
            points.iter().map(|(y, x)| (*x, *y)).collect()
        } else {
            Line::points_on_line(self.x1, self.y1, self.x2, k)
        }
    }

    fn points_on_line(x1: i32, y1: i32, x2: i32, k: f32) -> Vec<(i32, i32)> {
        let mut points = Vec::new();

        let range: Vec<i32>;
        if x1 > x2 {
            range = (x2..=x1).rev().collect();
        } else {
            range = (x1..=x2).collect();
        }

        let x0 = *range.get(0).unwrap() as f32;
        for x in range {
            points.push((x, y1 + ((x as f32 - x0) * k).round() as i32))
        }
        points
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn parse_line() {
        let input = "1,1 -> 3,1";
        let line = Line::new(input);

        assert_eq!(1, line.x1);
        assert_eq!(1, line.y1);
        assert_eq!(3, line.x2);
        assert_eq!(1, line.y2);

        assert_eq!(true, line.is_hori_vert());

        assert_eq!(vec![(1, 1), (2, 1), (3, 1)], line.points())
    }

    #[test]
    fn parse_line_vert() {
        let input = "9,9 -> 9,5";
        let line = Line::new(input);

        assert_eq!(9, line.x1);
        assert_eq!(9, line.y1);
        assert_eq!(9, line.x2);
        assert_eq!(5, line.y2);

        assert_eq!(true, line.is_hori_vert());

        assert_eq!(vec![(9, 9), (9, 8), (9, 7), (9, 6), (9, 5)], line.points())
    }

    #[test]
    fn parse_line_diag() {
        let input = "0,0 -> 1,1";
        let line = Line::new(input);

        assert_eq!(0, line.x1);
        assert_eq!(0, line.y1);
        assert_eq!(1, line.x2);
        assert_eq!(1, line.y2);

        assert_eq!(false, line.is_hori_vert());

        assert_eq!(vec![(0, 0), (1, 1)], line.points())
    }

    #[test]
    fn parse_line_diag_long() {
        let input = "4,3 -> 1,0";
        let line = Line::new(input);

        assert_eq!(4, line.x1);
        assert_eq!(3, line.y1);
        assert_eq!(1, line.x2);
        assert_eq!(0, line.y2);

        assert_eq!(false, line.is_hori_vert());

        assert_eq!(vec![(4, 3), (3, 2), (2, 1), (1, 0)], line.points())
    }

    #[test_case("sample1.txt" => is eq(5) ; "sample")]
    #[test_case("input.txt" => is eq(5576) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let lines = lines(input, false);

        let mut points: Vec<(i32, i32)> = lines.iter().flat_map(|l| l.points()).collect();
        points.sort();
        points
            .group_by(|l1, l2| l1 == l2)
            .map(|l| l.len() as i32)
            .filter(|count| *count > 1)
            .count() as i32
    }

    #[test_case("sample1.txt" => is eq(12) ; "sample")]
    #[test_case("input.txt" => is eq(18144) ; "input")]
    fn part2(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let lines = lines(input, true);

        let mut points: Vec<(i32, i32)> = lines.iter().flat_map(|l| l.points()).collect();
        points.sort();
        points
            .group_by(|l1, l2| l1 == l2)
            .map(|l| l.len() as i32)
            .filter(|count| *count > 1)
            .count() as i32
    }

    fn lines(input: String, allow_diagonal: bool) -> Vec<Line> {
        let lines: Vec<&str> = input.trim().lines().collect();
        lines
            .iter()
            .map(|s| Line::new(*s))
            .filter(|l| allow_diagonal || l.is_hori_vert())
            .collect()
    }
}
