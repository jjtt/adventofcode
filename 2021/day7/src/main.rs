use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn count_fuel(crabs: Vec<i32>, align_to: i32) -> i32 {
    crabs.iter().map(|c| (c - align_to).abs()).sum()
}

fn count_fuel_more(crabs: Vec<i32>, align_to: i32) -> i32 {
    crabs
        .iter()
        .map(|c| {
            let diff = (c - align_to).abs();
            diff * (1 + diff) / 2
        })
        .sum()
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

        count_fuel(crabs, align_to)
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

        count_fuel_more(crabs, align_to)
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
            let fuel = count_fuel(crabs.clone(), align_to);
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
            let fuel = count_fuel_more(crabs.clone(), align_to);
            if fuel < min_fuel {
                min_fuel = fuel;
                _chosen = align_to;
            }
        }

        min_fuel
    }
}
