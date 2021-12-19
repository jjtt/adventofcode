use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use test_case::test_case;

    use super::*;

    #[test_case("9" => is eq(9); "num")]
    #[test_case("[9,1]" => is eq(29); "0")]
    #[test_case("[[1,2],[[3,4],5]]" => is eq(143); "1")]
    #[test_case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" => is eq(1384); "2")]
    #[test_case("[[[[1,1],[2,2]],[3,3]],[4,4]]" => is eq(445); "3")]
    #[test_case("[[[[3,0],[5,3]],[4,4]],[5,5]]" => is eq(791); "4")]
    #[test_case("[[[[5,0],[7,4]],[5,5]],[6,6]]" => is eq(1137); "5")]
    #[test_case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" => is eq(3488); "6")]
    fn magnitude(input: &str) -> u32 {
        mag(input)
    }

    fn mag(input: &str) -> u32 {
        let l = input.len();
        if l == 1 {
            input.parse().unwrap()
        } else {
            let mut split = 0;
            let mut brackets = 0;
            for i in 1..l - 1 {
                match &input[i..i + 1] {
                    "[" => brackets += 1,
                    "]" => brackets -= 1,
                    "," => {
                        if brackets == 0 {
                            split = i;
                            break;
                        }
                    }
                    _ => continue,
                }
            }
            3 * mag(&input[1..split]) + 2 * mag(&input[split + 1..l - 1])
        }
    }

    #[test_case("sample1.txt" => is eq(4140); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> u32 {
        0
    }
}
