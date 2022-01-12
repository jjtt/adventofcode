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

fn neighbours_to_index(
    image: &HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    image_is_negative: bool,
) -> usize {
    let index = (((image_is_negative ^ image.contains(&(x - 1, y - 1))) as usize) << 8)
        + (((image_is_negative ^ image.contains(&(x, y - 1))) as usize) << 7)
        + (((image_is_negative ^ image.contains(&(x + 1, y - 1))) as usize) << 6)
        + (((image_is_negative ^ image.contains(&(x - 1, y))) as usize) << 5)
        + (((image_is_negative ^ image.contains(&(x, y))) as usize) << 4)
        + (((image_is_negative ^ image.contains(&(x + 1, y))) as usize) << 3)
        + (((image_is_negative ^ image.contains(&(x - 1, y + 1))) as usize) << 2)
        + (((image_is_negative ^ image.contains(&(x, y + 1))) as usize) << 1)
        + (((image_is_negative ^ image.contains(&(x + 1, y + 1))) as usize) << 0);
    index
}

fn apply(
    algo: &Vec<bool>,
    image: HashSet<(i32, i32)>,
    min: i32,
    max: i32,
    image_is_negative: bool,
) -> (HashSet<(i32, i32)>, bool) {
    let mut out = HashSet::new();

    let makes_negatives = *algo.first().unwrap();

    for x in min - 1..=max + 1 {
        for y in min - 1..=max + 1 {
            let new_value = algo[neighbours_to_index(&image, x, y, image_is_negative)];
            if !makes_negatives {
                if !image_is_negative {
                    if new_value {
                        out.insert((x, y));
                    }
                } else {
                    todo!();
                }
            } else {
                if !image_is_negative {
                    if !new_value {
                        out.insert((x, y));
                    }
                } else {
                    if new_value {
                        out.insert((x, y));
                    }
                }
            }
        }
    }

    (
        out,
        if makes_negatives {
            !image_is_negative
        } else {
            image_is_negative
        },
    )
}

fn print(image: &HashSet<(i32, i32)>, min: i32, max: i32, image_is_negative: bool) {
    dbg!((image_is_negative, image.len()));
    for y in min..=max {
        for x in min..=max {
            let contains = image.contains(&(x, y));
            if contains {
                if image_is_negative {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                if image_is_negative {
                    print!("¤");
                } else {
                    print!(",");
                }
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn index_for_center_in_sample() {
        let (_, image) = parse_input("sample1.txt");

        assert_eq!(34, neighbours_to_index(&image, 2, 2, false));
        assert_eq!(477, neighbours_to_index(&image, 2, 2, true));
    }

    #[test]
    fn index_for_center_in_sample2() {
        let (_, image) = parse_input("sample2.txt");

        assert_eq!(16, neighbours_to_index(&image, 0, 0, false));
    }

    #[test]
    fn index_for_center_in_sample3() {
        let (_, image) = parse_input("sample3.txt");

        assert!(image.is_empty());
        assert_eq!(0, neighbours_to_index(&image, 0, 0, false));
        assert_eq!(511, neighbours_to_index(&image, 0, 0, true));

        assert_eq!(0, neighbours_to_index(&image, 1, 1, false));
        assert_eq!(511, neighbours_to_index(&image, 1, 1, true));
        assert_eq!(0, neighbours_to_index(&image, -100, 100, false));
        assert_eq!(511, neighbours_to_index(&image, -100, 100, true));
    }

    #[test]
    fn empty_image_applied() {
        let mut algo = vec![true];
        algo.extend(vec![false; 511]);
        assert_eq!(512, algo.len());
        let image = HashSet::new();

        print(&image, -10, 10, false);
        let (image, negative) = apply(&algo, image, 0, 0, false);
        assert!(negative);
        print(&image, -10, 10, negative);
        assert!(image.is_empty());
    }

    #[test]
    fn empty_image_applied_as_negative() {
        let mut algo = vec![true];
        algo.extend(vec![false; 511]);
        assert_eq!(512, algo.len());
        let image = HashSet::new();

        print(&image, -10, 10, true);
        let (image, negative) = apply(&algo, image, 0, 0, true);
        assert!(!negative);
        print(&image, -10, 10, negative);
        assert!(image.is_empty());
    }

    #[test]
    fn indices_for_input() {
        let (_, image) = parse_input("input.txt");

        assert_eq!(431, neighbours_to_index(&image, 98, 98, false));
        assert_eq!(31, neighbours_to_index(&image, 1, 1, false));
        assert_eq!(0, neighbours_to_index(&image, -1, -1, false));
        assert_eq!(511, neighbours_to_index(&image, -1, -1, true));
        assert_eq!(4, neighbours_to_index(&image, 100, -1, false));
        assert_eq!(507, neighbours_to_index(&image, 100, -1, true));
    }

    #[test_case("sample1.txt" => is eq(35); "sample1")]
    #[test_case("sample2.txt" => is eq(1); "sample2")]
    #[test_case("sample3.txt" => is eq(0); "sample3")]
    #[test_case("input.txt" => is eq(5306); "input")]
    fn part1(input: &str) -> usize {
        let (algo, image) = parse_input(input);

        let min = *image.iter().map(|(x, _)| x).min().unwrap_or(&0);
        let max = *image.iter().map(|(x, _)| x).max().unwrap_or(&0);

        let negative = false;
        print(&image, min - 10, max + 10, negative);
        let (image, negative) = apply(&algo, image, min, max, negative);
        print(&image, min - 10, max + 10, negative);
        let (image, negative) = apply(&algo, image, min - 1, max + 1, negative);
        print(&image, min - 10, max + 10, negative);

        image.len()
    }

    #[test_case("sample1.txt" => is eq(3351); "sample1")]
    #[test_case("input.txt" => is eq(17497); "input")]
    fn part2(input: &str) -> usize {
        let (algo, mut image) = parse_input(input);

        let min = *image.iter().map(|(x, _)| x).min().unwrap_or(&0);
        let max = *image.iter().map(|(x, _)| x).max().unwrap_or(&0);

        let mut negative = false;
        for d in 0..50 {
            let (i, n) = apply(&algo, image, min - d, max + d, negative);
            image = i;
            negative = n;
        }

        image.len()
    }
}
