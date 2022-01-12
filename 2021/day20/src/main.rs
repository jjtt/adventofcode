#[macro_use]
extern crate scan_fmt;

use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> (Vec<bool>, HashSet<(i32, i32)>) {
    let result = read_to_string(input).unwrap();
    let mut lines = result.lines();

    let algorithm = lines.next().unwrap().chars().map(|c| c == '#').collect();

    assert!(lines.next().unwrap().is_empty());

    let mut image = HashSet::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i32, y as i32));
            }
        }
    }

    (algorithm, image)
}

fn neighbours_to_index(image: &HashSet<(i32, i32)>, x: i32, y: i32) -> usize {
    ((image.contains(&(x - 1, y - 1)) as usize) << 8)
        + ((image.contains(&(x, y - 1)) as usize) << 7)
        + ((image.contains(&(x + 1, y - 1)) as usize) << 6)
        + ((image.contains(&(x - 1, y)) as usize) << 5)
        + ((image.contains(&(x, y)) as usize) << 4)
        + ((image.contains(&(x + 1, y)) as usize) << 3)
        + ((image.contains(&(x - 1, y + 1)) as usize) << 2)
        + ((image.contains(&(x, y + 1)) as usize) << 1)
        + ((image.contains(&(x + 1, y + 1)) as usize) << 0)
}

fn apply(algo: &Vec<bool>, image: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let x_min = image.iter().map(|(x, _)| x).min().unwrap();
    let x_max = image.iter().map(|(x, _)| x).max().unwrap();
    let y_min = image.iter().map(|(_, y)| y).min().unwrap();
    let y_max = image.iter().map(|(_, y)| y).max().unwrap();

    let mut out = HashSet::new();

    for x in x_min - 1..=x_max + 1 {
        for y in y_min - 1..=y_max + 1 {
            if algo[neighbours_to_index(&image, x, y)] {
                out.insert((x, y));
            }
        }
    }

    out
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use test_case::test_case;

    use super::*;

    #[test]
    fn scratch() {
        let a = ['a', 'b', 'c'];

        let mut foo = a.iter();

        //foo.next();

        let mut iter = foo.enumerate();

        assert_eq!(iter.next(), Some((0, &'a')));
        assert_eq!(iter.next(), Some((1, &'b')));
        assert_eq!(iter.next(), Some((2, &'c')));
        assert_eq!(iter.next(), None);

        assert_eq!(1, true as usize);
        assert_eq!(0, false as usize);
        assert_eq!(2, (true as usize) << 1);
        assert_eq!(0, (false as usize) << 1);
    }

    #[test]
    fn index_for_center_in_sample() {
        let (_, image) = parse_input("sample1.txt");

        assert_eq!(34, neighbours_to_index(&image, 2, 2));
    }

    #[test_case("sample1.txt" => is eq(35); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let (algo, image) = parse_input(input);

        let image = apply(&algo, image);
        let image = apply(&algo, image);

        image.len()
    }
}
