use std::fs::read_to_string;
use std::ops::ControlFlow;

fn main() {
    println!("Hello, world!");
}

fn find_closing(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn validate(chunk: &str) -> Option<(char, String)> {
    match chunk
        .chars()
        .try_fold(String::new(), |mut stack, c| match find_closing(c) {
            Some(closing) => {
                stack.insert(0, closing);
                ControlFlow::Continue(stack)
            }
            None => {
                if stack.remove(0) != c {
                    ControlFlow::Break(c)
                } else {
                    ControlFlow::Continue(stack)
                }
            }
        }) {
        ControlFlow::Break(c) => Some((c, "".to_string())),
        ControlFlow::Continue(stack) => match stack.as_str() {
            "" => None,
            stack => Some(('_', stack.to_string())),
        },
    }
}

fn score_syntax_check((invalid, _): (char, String)) -> Option<u32> {
    match invalid {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn score_autocompletion((_, completion): (char, String)) -> Option<u64> {
    completion
        .chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Invalid character in autocompletion: {}", c),
        })
        .reduce(|sum, cur| sum * 5 + cur)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("(]" => is eq(']') ; "1")]
    #[test_case("{()()()>" => is eq('>') ; "2")]
    #[test_case("(((()))}" => is eq('}') ; "3")]
    #[test_case("<([]){()}[{}])" => is eq(')') ; "4")]
    fn corrupt(chunk: &str) -> char {
        validate(chunk).unwrap().0
    }

    #[test_case("([])" => is none() ; "1")]
    #[test_case("{()()()}" => is none() ; "2")]
    #[test_case("<([{}])>" => is none() ; "3")]
    #[test_case("[<>({}){}[([])<>]]" => is none() ; "4")]
    #[test_case("(((((((((())))))))))" => is none() ; "5")]
    fn valid(chunk: &str) -> Option<(char, String)> {
        validate(chunk)
    }

    #[test_case("[({(<(())[]>[[{[]{<()<>>" => is eq("}}]])})]") ; "1")]
    #[test_case("[(()[<>])]({[<{<<[]>>(" => is eq(")}>]})") ; "2")]
    #[test_case("(((({<>}<{<{<>}{[]{[]{}" => is eq("}}>}>))))") ; "3")]
    #[test_case("{<[[]]>}<{[{[{[]{()[[[]" => is eq("]]}}]}]}>") ; "4")]
    #[test_case("<{([{{}}[<[[[<>{}]]]>[]]" => is eq("])}>") ; "5")]
    fn complete(chunk: &str) -> String {
        validate(chunk).unwrap().1
    }

    #[test_case("sample1.txt" => is eq(26397) ; "sample")]
    #[test_case("input.txt" => is eq(367227) ; "input")]
    fn part1(input: &str) -> u32 {
        read_to_string(input)
            .unwrap()
            .lines()
            .filter_map(validate)
            .filter_map(score_syntax_check)
            .sum()
    }

    #[test_case("sample1.txt" => is eq(288957) ; "sample")]
    #[test_case("input.txt" => is eq(3583341858) ; "input")]
    fn part2(input: &str) -> u64 {
        let mut scores: Vec<u64> = read_to_string(input)
            .unwrap()
            .lines()
            .filter_map(validate)
            .filter_map(score_autocompletion)
            .collect();

        scores.sort();

        *scores.get(scores.len() / 2).unwrap()
    }
}
