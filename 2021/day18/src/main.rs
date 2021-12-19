use std::collections::LinkedList;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Value {
    depth: u32,
    value: u32,
}

fn parse(input: &str) -> LinkedList<Value> {
    let l = input.len();
    let mut list = LinkedList::new();
    let mut depth = 0;
    for i in 0..l {
        match &input[i..i + 1] {
            "[" => depth += 1,
            "]" => depth -= 1,
            "," => (),
            num => list.push_back(Value{depth, value: num.parse().unwrap()}),
        }
    }
    list
}

fn find_middle_comma(input: &str) -> usize {
    let l = input.len();
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
    split
}

fn mag(input: &str) -> u32 {
    let l = input.len();
    if l == 1 {
        input.parse().unwrap()
    } else {
        let split = find_middle_comma(input);
        3 * mag(&input[1..split]) + 2 * mag(&input[split + 1..l - 1])
    }
}

fn add(first: &str, second: &str) -> String {
    format!("[{},{}]", first, second)
}

#[cfg(test)]
mod test {
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

    #[test_case("[1,2]","[[3,4],5]" => is eq("[[1,2],[[3,4],5]]"); "simple add")]
    fn adding(first: &str, second: &str) -> String {
        add(first, second)
    }

    #[test_case("[[[[[9,8],1],2],3],4]","[[[[0,9],2],3],4]" => is eq(true); "first")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]","[7,[6,[5,[7,0]]]]" => is eq(true); "last")]
    #[test_case("[[6,[5,[4,[3,2]]]],1]","[[6,[5,[7,0]]],3]" => is eq(true); "in middle")]
    #[test_case("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]","[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]" => is eq(true); "one at a time")]
    #[test_case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]","[[3,[2,[8,0]]],[9,[5,[7,0]]]]" => is eq(true); "last2")]
    fn exploding(input: &str, out: &str) -> bool {
        assert_eq!(explode(parse(input)), parse(out));
        true
    }

    fn explode(snailfishnum: LinkedList<Value>) -> LinkedList<Value> {
        let mut out = LinkedList::new();

        let mut it = snailfishnum.iter();

        let mut prev: Option<&Value> = None;
        let mut first = it.next().unwrap();
        while let Some(second) = it.next() {
            if first.depth == second.depth && first.depth > 4 {
                let next = it.next();
                if prev.is_some() {
                    let new_prev = Value{depth: prev.unwrap().depth, value: prev.unwrap().value + first.value};
                    out.push_back(new_prev);
                }

                out.push_back(Value{depth: first.depth-1, value: 0});

                if next.is_some() {
                    let new_next = Value{depth: next.unwrap().depth, value: next.unwrap().value + second.value};
                    out.push_back(new_next);
                }

                break;
            }
            if (prev.is_some()) {
                out.push_back(prev.unwrap().clone());
            }
            prev = Some(first);
            first = second;
        }

        for rest in it {
            out.push_back(rest.clone());
        }

        out
    }


    #[test_case("sample1.txt" => is eq(4140); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> u32 {
        0
    }
}
