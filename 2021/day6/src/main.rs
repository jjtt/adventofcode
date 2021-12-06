use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(5934) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        0
    }
}
