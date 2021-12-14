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

fn print(dots: &HashSet<(i32, i32)>) {
    let minx = *dots.iter().map(|(x, _)| x).min().unwrap();
    let maxx = *dots.iter().map(|(x, _)| x).max().unwrap();
    let miny = *dots.iter().map(|(_, y)| y).min().unwrap();
    let maxy = *dots.iter().map(|(_, y)| y).max().unwrap();

    for y in miny..=maxy {
        for x in minx..=maxx {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            };
        }
        println!();
    }
    println!();
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

    #[test_case("sample1.txt" => is eq(16) ; "sample1")]
    #[test_case("input.txt" => is eq(95) ; "input")]
    #[test_case("inputplusplus.txt" => is eq(95) ; "inputplusplus")]
    fn part2(input: &str) -> usize {
        let (mut dots, folds) = game_from_input(input);

        for f in folds {
            dots = fold(dots.clone(), f);
        }

        // Answer is the capital letters printed here
        print(&dots);

        dots.len()
    }
}
