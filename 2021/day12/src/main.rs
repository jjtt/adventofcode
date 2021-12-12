use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn octos_from_input(input: &str) -> Vec<Vec<u8>> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(10) ; "sample1")]
    #[test_case("sample2.txt" => is eq(226) ; "sample2")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> i32 {
        let mut octos = octos_from_input(input);

        let mut flashes = 0;

        for _ in 0..100 {
            step(&mut octos);

            flashes += flash(&mut octos);
        }

        flashes
    }
}
