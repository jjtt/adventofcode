use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn game_from_input(input: &str) -> (HashSet<(i32, i32)>, Vec<(i32, i32)>) {
    let mut dots = HashSet::new();
    let mut folds = vec![];
    let mut folding = false;
    for line in read_to_string(input).unwrap().lines() {
        if line == "" {
            folding = true;
        } else {
            if folding {
                let (a, n) = line.split("=").collect_tuple().unwrap();
                let num = n.parse().unwrap();
                folds.push(if a.ends_with("x") { (0, num) } else { (1, num) });
            } else {
                dots.insert(
                    line.split(",")
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
            }
        }
    }

    (dots, folds)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(17) ; "sample1")]
    #[test_case("input.txt" => is eq(693) ; "input")]
    fn part1(input: &str) -> usize {
        let (dots, folds) = game_from_input(input);

        let dots = fold(dots, *folds.first().unwrap());

        dots.len()
    }

    fn fold(dots: HashSet<(i32, i32)>, (axis, num): (i32, i32)) -> HashSet<(i32, i32)> {
        let mut folded = HashSet::new();

        for (x, y) in dots {
            if axis == 1 {
                if y < num {
                    folded.insert((x, y));
                } else {
                    folded.insert((x, num - (y - num)));
                }
            } else {
                if x < num {
                    folded.insert((x, y));
                } else {
                    folded.insert((num - (x - num), y));
                }
            }
        }

        folded
    }
}
