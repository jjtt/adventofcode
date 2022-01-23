use std::fs::read_to_string;



fn parse_cucumber_map(input: String) -> _ {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;


    #[test_case("sample1.txt" => is eq(58); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let state = parse_cucumber_map(read_to_string(input).unwrap());

        let mut moves = 0;
        let mut moved = true;
        while !moved {
            moves += 1;

            moved = true;
        }

        moves
    }
}
