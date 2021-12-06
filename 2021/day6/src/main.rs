use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(5934) ; "sample")]
    #[test_case("input.txt" => is eq(391888) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let mut fishy_fish: Vec<i32> = input
            .trim()
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect();

        //dbg!(&fishy_fish);

        for day in 0..80 {
            fishy_fish = go_forth(fishy_fish.clone());
            //dbg!(&fishy_fish);
        }

        fishy_fish.len() as i32
    }

    fn go_forth(mut fishy_fish: Vec<i32>) -> Vec<i32> {
        for f in 0..fishy_fish.len() {
            let fish = fishy_fish.get_mut(f).unwrap();
            if *fish == 0 {
                *fish = 6;
                fishy_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        fishy_fish
    }
}
