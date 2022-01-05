#[macro_use]
extern crate scan_fmt;
use std::fs::read_to_string;

fn parse_sensors(input: &str) -> Vec<(i32, Vec<Point>)> {
    let s = read_to_string(input).unwrap();

    s.split("\n\n").map(parse_sensor).collect()
}

fn parse_sensor(input: &str) -> (i32, Vec<Point>) {
    let (s, c) = input.split_once("\n").unwrap();

    (
        scan_fmt!(s, "--- scanner {d} ---", i32).unwrap(),
        parse_coordinates(c),
    )
}


#[cfg(test)]
mod test {
    use indoc::indoc;
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(0); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let sensors = parse_sensors(input);

        0
    }
}
