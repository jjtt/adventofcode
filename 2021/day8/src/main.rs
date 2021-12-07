use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn x_from_input(input: &str) -> Vec<i32> {
    read_to_string(input).unwrap()
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(0) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> i32 {
        let x = x_from_input(input);

        0
    }
}
