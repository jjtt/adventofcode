use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn x_from_input(input: &str) -> Vec<Vec<Vec<String>>> {
    read_to_string(input)
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split("|")
                .map(str::to_string)
                .map(|s| s.trim().split(" ").map(str::to_string).collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(26) ; "sample")]
    #[test_case("input.txt" => is eq(479) ; "input")]
    fn part1(input: &str) -> usize {
        let x = x_from_input(input);

        let mut sum: usize = 0;
        let easy_ones = vec![7, 4, 3, 2];
        for l in x {
            let output = l.get(1).unwrap();
            sum += output
                .iter()
                .map(String::len)
                .filter(|o| easy_ones.contains(&o))
                .count();
        }

        sum
    }
}
