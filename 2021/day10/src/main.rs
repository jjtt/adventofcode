use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn validate(chunk: &str) -> Option<char> {
    let allowed_chars: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();
    let mut stack = vec![];
    for c in chunk.chars() {
        if allowed_chars.contains_key(&c) {
            stack.push(allowed_chars.get(&c).unwrap());
        } else {
            let closing = stack.pop().unwrap();
            if *closing != c {
                return Some(c);
            }
        }
    }
    None
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
        validate(chunk).unwrap()
    }

    #[test_case("([])" => is none() ; "1")]
    #[test_case("{()()()}" => is none() ; "2")]
    #[test_case("<([{}])>" => is none() ; "3")]
    #[test_case("[<>({}){}[([])<>]]" => is none() ; "4")]
    #[test_case("(((((((((())))))))))" => is none() ; "5")]
    fn valid(chunk: &str) -> Option<char> {
        validate(chunk)
    }

    #[test_case("sample1.txt" => is eq(26397) ; "sample")]
    #[test_case("input.txt" => is eq(367227) ; "input")]
    fn part1(input: &str) -> u32 {
        let mut score = 0;
        for line in read_to_string(input).unwrap().lines() {
            score += match validate(line) {
                Some(')') => 3,
                Some(']') => 57,
                Some('}') => 1197,
                Some('>') => 25137,
                _ => 0,
            };
        }
        score
    }
}
