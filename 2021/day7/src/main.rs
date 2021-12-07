use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn count_fuel<F>(crabs: Vec<i32>, cost: F) -> i32
where
    F: Fn(&i32) -> i32,
{
    crabs.iter().map(cost).sum()
}

fn distance(from: i32) -> Box<dyn Fn(&i32) -> i32> {
    Box::new(move |c| (c - from).abs())
}

fn triangular_number(from: i32) -> Box<dyn Fn(&i32) -> i32> {
    Box::new(move |c| {
        let diff = (c - from).abs();
        diff * (1 + diff) / 2
    })
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt", 2 => is eq(37) ; "sample to 2")]
    #[test_case("sample1.txt", 1 => is eq(41) ; "sample to 1")]
    #[test_case("sample1.txt", 3 => is eq(39) ; "sample to 3")]
    #[test_case("sample1.txt", 10 => is eq(71) ; "sample to 10")]
    fn partial(input: &str, align_to: i32) -> i32 {
        let input = read_to_string(input).unwrap();

        let crabs: Vec<i32> = input
            .trim()
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect();

        count_fuel(crabs, distance(align_to))
    }

    #[test_case("sample1.txt", 5 => is eq(168) ; "sample to 5")]
    #[test_case("sample1.txt", 2 => is eq(206) ; "sample to 2")]
    fn partial2(input: &str, align_to: i32) -> i32 {
        let input = read_to_string(input).unwrap();

        let crabs: Vec<i32> = input
            .trim()
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect();

        count_fuel(crabs, triangular_number(align_to))
    }

    #[test_case("sample1.txt" => is eq(37) ; "sample")]
    #[test_case("input.txt" => is eq(356922) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let crabs: Vec<i32> = input
            .trim()
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect();

        let mut min_fuel = i32::MAX;
        let mut _chosen;
        for align_to in *crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap() {
            let fuel = count_fuel(crabs.clone(), distance(align_to));
            if fuel < min_fuel {
                min_fuel = fuel;
                _chosen = align_to;
            }
        }

        min_fuel
    }

    #[test_case("sample1.txt" => is eq(168) ; "sample")]
    #[test_case("input.txt" => is eq(100347031) ; "input")]
    fn part2(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let crabs: Vec<i32> = input
            .trim()
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect();

        let mut min_fuel = i32::MAX;
        let mut _chosen;
        for align_to in *crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap() {
            let fuel = count_fuel(crabs.clone(), triangular_number(align_to));
            if fuel < min_fuel {
                min_fuel = fuel;
                _chosen = align_to;
            }
        }

        min_fuel
    }
}
