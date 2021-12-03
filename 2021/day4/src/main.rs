use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(0) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> u32 {
        let input = read_to_string(input).unwrap();

        let foo: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        dbg!(foo);

        0
    }
}
