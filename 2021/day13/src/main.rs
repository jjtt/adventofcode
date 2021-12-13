use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn game_from_input(input: &str) -> (HashSet<(i32, i32)>, Vec<(i32, i32)>) {
    let dots = HashSet::new();
    let folds = vec![];
    let mut folding = false;
    for line in read_to_string(input).unwrap().lines() {
        if line == "" {
            folding = true;
        }

        if folding {
            // folds
        } else {
            // dots
        }
    }

    (dots, folds)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(17) ; "sample1")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> usize {
        let (dots, folds) = game_from_input(input);

        0
    }
}
