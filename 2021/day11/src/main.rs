use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(1656) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> i128 {
        read_to_string(input)
            .unwrap()
            .lines()
            .map(|l| l.parse::<i128>().unwrap())
            .sum()
    }
}
