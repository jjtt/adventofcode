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

fn neighbours_to_index(image: &HashSet<(i32, i32)>, x: i32, y: i32, negative: bool) -> usize {
    let index = ((image.contains(&(x - 1, y - 1)) as usize) << 8)
        + ((image.contains(&(x, y - 1)) as usize) << 7)
        + ((image.contains(&(x + 1, y - 1)) as usize) << 6)
        + ((image.contains(&(x - 1, y)) as usize) << 5)
        + ((image.contains(&(x, y)) as usize) << 4)
        + ((image.contains(&(x + 1, y)) as usize) << 3)
        + ((image.contains(&(x - 1, y + 1)) as usize) << 2)
        + ((image.contains(&(x, y + 1)) as usize) << 1)
        + ((image.contains(&(x + 1, y + 1)) as usize) << 0);

    if negative {
        (!index) & 0b111111111
    } else {
        index
    }
}

fn apply(
    algo: &Vec<bool>,
    image: HashSet<(i32, i32)>,
    min: i32,
    max: i32,
    negative: bool,
) -> (HashSet<(i32, i32)>, bool) {
    let mut out = HashSet::new();

    let makes_negatives = *algo.first().unwrap();

    for x in min - 1..=max + 1 {
        for y in min - 1..=max + 1 {
            let new_value = algo[neighbours_to_index(&image, x, y, negative)];
            if !makes_negatives && new_value {
                out.insert((x, y));
            } else if makes_negatives && !new_value {
                out.insert((x, y));
            }
        }
    }

    (out, makes_negatives)
}

fn print(image: &HashSet<(i32, i32)>, min: i32, max: i32, negative: bool) {
    for y in min..=max {
        for x in min..=max {
            let contains = image.contains(&(x, y));
            if contains {
                if negative {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                if negative {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    println!();
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

        assert_eq!(34, neighbours_to_index(&image, 2, 2, false));
        assert_eq!(477, neighbours_to_index(&image, 2, 2, true));
    }

    #[test]
    fn indices_for_input() {
        let (_, image) = parse_input("input.txt");

        assert_eq!(431, neighbours_to_index(&image, 98, 98, false));
        assert_eq!(31, neighbours_to_index(&image, 1, 1, false));
    }

    #[test_case("sample1.txt" => is eq(35); "sample1")]
    #[test_case("sample2.txt" => is eq(0); "sample2")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let (algo, image) = parse_input(input);

        let min = *image.iter().map(|(x, _)| x).min().unwrap();
        let max = *image.iter().map(|(x, _)| x).max().unwrap();

        let negative = false;
        print(&image, min, max, negative);
        let (image, negative) = apply(&algo, image, min, max, negative);
        print(&image, min - 1, max + 1, negative);
        let (image, negative) = apply(&algo, image, min - 1, max + 1, negative);
        print(&image, min - 2, max + 2, negative);

        image.len()
    }
}
