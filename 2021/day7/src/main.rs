use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(37) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> usize {
        let input = read_to_string(input).unwrap();

        let crabs: Vec<i32> = input
            .trim()
            .split(",")
            .map(|f| f.parse().unwrap()).collect();

        0
    }
}
