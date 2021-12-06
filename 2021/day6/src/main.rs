use cached::proc_macro::cached;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn fishes(mut fishy_fish: Vec<i8>, days: i32) -> usize {
    for _ in 0..days {
        fishy_fish = go_forth(fishy_fish.clone());
    }

    fishy_fish.len()
}

fn go_forth(mut fishy_fish: Vec<i8>) -> Vec<i8> {
    for f in 0..fishy_fish.len() {
        let fish = fishy_fish.get_mut(f).unwrap();
        if *fish == 0 {
            *fish = 6;
            fishy_fish.push(8);
        } else {
            *fish -= 1;
        }
    }
    fishy_fish
}

#[cached]
fn fish_rec(value: i8, days: i32) -> usize {
    let remaining = days - 1 - value as i32;
    if remaining < 0 {
        1
    } else {
        fish_rec(6, remaining) + fish_rec(8, remaining)
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn one_fish() {
        assert_eq!(fishes(vec![1], 1), fish_rec(1, 1));
        for days in 1..80 {
            let init_fish = 0;
            assert_eq!(
                fishes(vec![init_fish], days),
                fish_rec(init_fish, days),
                "Not equal after {} days",
                days
            );
        }
    }

    #[test_case("sample1.txt" => is eq(5934) ; "sample")]
    #[test_case("input.txt" => is eq(391888) ; "input")]
    fn part1(input: &str) -> usize {
        let input = read_to_string(input).unwrap();

        input
            .trim()
            .split(",")
            .map(|f| fish_rec(f.parse().unwrap(), 80))
            .sum()
    }

    #[test_case("sample1.txt" => is eq(26984457539) ; "sample")]
    #[test_case("input.txt" => is eq(1754597645339) ; "input")]
    fn part2(input: &str) -> usize {
        let input = read_to_string(input).unwrap();

        input
            .trim()
            .split(",")
            .map(|f| fish_rec(f.parse().unwrap(), 256))
            .sum()
    }
}
