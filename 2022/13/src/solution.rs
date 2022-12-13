use json::{parse, JsonValue};
use std::cmp::Ordering;
use std::fs::read_to_string;

fn compare(left: &JsonValue, right: &JsonValue) -> Ordering {
    assert!(left.is_array() || left.is_number());
    assert!(right.is_array() || right.is_number());

    if left.is_number() && right.is_number() {
        return right.as_i64().unwrap().cmp(&left.as_i64().unwrap());
    }

    let left_in_array = JsonValue::Array(vec![left.clone()]);
    let right_in_array = JsonValue::Array(vec![right.clone()]);
    let left = if left.is_number() {
        &left_in_array
    } else {
        left
    };
    let right = if right.is_number() {
        &right_in_array
    } else {
        right
    };

    match (left, right) {
        (left, right) if left.is_empty() && right.is_empty() => Ordering::Equal,
        (empty, _) if empty.is_empty() => Ordering::Greater,
        (_, empty) if empty.is_empty() => Ordering::Less,

        (left, right) => {
            let count = left.len().min(right.len());
            for i in 0..count {
                let left = &left[i];
                let right = &right[i];
                let result = compare(left, right);
                if result != Ordering::Equal {
                    return result;
                }
            }
            right.len().cmp(&left.len())
        }
    }
}

fn order_ok(left: &str, right: &str) -> bool {
    Ordering::Greater == compare(&parse(left).unwrap(), &parse(right).unwrap())
}

pub fn part1(input: &str) -> usize {
    read_to_string(input)
        .unwrap()
        .split("\n\n")
        .enumerate()
        .map(|(index, pair)| (index + 1, pair.split('\n')))
        .filter_map(|(index, mut s)| {
            if order_ok(s.next().unwrap(), s.next().unwrap()) {
                Some(index)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets: Vec<JsonValue> = read_to_string(input)
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| parse(s).unwrap())
        .collect();

    packets.sort_by(compare);
    packets.reverse();

    let first = parse("[[2]]").unwrap();
    let second = parse("[[6]]").unwrap();

    let first_index = packets
        .binary_search_by(|i| compare(&first, i))
        .unwrap_err();
    let second_index = packets
        .binary_search_by(|i| compare(&second, i))
        .unwrap_err();

    (first_index.min(second_index) + 1) * (first_index.max(second_index) + 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn json_parsing() {
        let lists = json::parse("[1,[2,[]]]").unwrap();

        dbg!(lists);
    }

    #[test_case("[1]", "[1]" => Ordering::Equal; "ones")]
    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]" => Ordering::Greater; "s1")]
    #[test_case("[[1],[2,3,4]]", "[[1],4]" => Ordering::Greater; "s2")]
    #[test_case("[9]", "[[8,7,6]]" => Ordering::Less; "s3")]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]" => Ordering::Greater; "s4")]
    #[test_case("[7,7,7,7]", "[7,7,7]" => Ordering::Less; "s5")]
    #[test_case("[]", "[3]" => Ordering::Greater; "s6")]
    #[test_case("[[[]]]", "[[]]" => Ordering::Less; "s7")]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]" => Ordering::Less; "s8")]
    fn comparing(left: &str, right: &str) -> Ordering {
        let left = json::parse(left).unwrap();
        let right = json::parse(right).unwrap();

        compare(&left, &right)
    }

    #[test]
    fn ok_ordering() {
        assert!(order_ok("[]", "1"));
        assert!(!order_ok("[2]", "1"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(140, part2("sample.txt"));
    }
}
