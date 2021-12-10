use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn validate(chunk: &str) -> Option<(char, String)> {
    let allowed_chars: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();
    let mut stack = String::new();
    for c in chunk.chars() {
        if allowed_chars.contains_key(&c) {
            stack.insert(0, *allowed_chars.get(&c).unwrap());
        } else {
            let closing = stack.remove(0);
            if closing != c {
                return Some((c, "".to_string()));
            }
        }
    }
    if !stack.is_empty() {
        return Some(('_', stack));
    }
    None
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
            _ => 0,
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
