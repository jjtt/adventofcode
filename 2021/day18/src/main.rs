use std::collections::LinkedList;
use std::fs::read_to_string;
use num_integer::Integer;
use crate::SepOrValue::{Sep, Value};

fn main() {
    println!("Hello, world!");
}

type Link = Option<Box<Node>>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    left: Link,
    right: Link,
    value: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tree {
    root: Node,
}

#[derive(Clone, Copy, Debug)]
enum SepOrValue {
    Sep(char),
    Value(i32),
}

fn parseTree(input: &str) -> Tree {
    Tree {
        root: *parse(input).unwrap(),
    }
}

fn parse(input: &str) -> Link {
    if ! input.contains(",") {
        return Some(Box::new(Node{
            left: None,
            right: None,
            value: Some(input.parse().unwrap()),
        }))
    }

    let middle = find_middle_comma(input);
    Some(Box::new(Node {
        left: parse(&input[1..middle]),
        right: parse(&input[middle + 1..input.len()-1]),
        value: None,
    }))
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

fn mag(input: &str) -> i32 {
    let l = input.len();
    if l == 1 {
        input.parse().unwrap()
    } else {
        let split = find_middle_comma(input);
        3 * mag(&input[1..split]) + 2 * mag(&input[split + 1..l - 1])
    }
}

fn mag2(node: Node) -> i32 {
    if node.value.is_some() {
        return node.value.unwrap()
    } else {
        3 * mag2(*node.left.unwrap()) + 2 * mag2(*node.right.unwrap())
    }
}

fn splitTree(tree: Tree) -> Tree {
    let (_has_split, node) = split(false, tree.root);
    Tree {
        root: node
    }
}

fn split(has_split: bool, node: Node) -> (bool,Node) {
    if node.value.is_some() {
        let value = node.value.unwrap();
        if !has_split && value >= 10 {
            (true,
            Node {
                left: Some(Box::new(Node {
                    left: None,
                    right: None,
                    value: Some(value.div_floor(&2)),
                })),
                right: Some(Box::new(Node {
                    left: None,
                    right: None,
                    value: Some(value.div_ceil(&2)),
                })),
                value: None,
            })
        } else {
            (has_split, node.clone())
        }
    } else {
        let mut new_has_split = has_split;
        let (new_has_split, new_left) = split(new_has_split, *node.left.unwrap());
        let (new_has_split, new_right) = split(new_has_split, *node.right.unwrap());
        (new_has_split,
        Node{
            left: Some(Box::new(new_left)),
            right: Some(Box::new(new_right)),
            value: None,
        })
    }
}

fn explodeTree(tree: Tree) -> Tree {
    let mut flat = Vec::new();
    flatten(tree.root, &mut flat);

    let mut out = Vec::new();
    let mut depth = 0;
    let mut exploded = false;
    let mut add_to_next = None;
    let mut prev_index = None;
    let mut i = 0;
    while i < flat.len() {
        let mut cur = *flat.get(i).unwrap();
        match cur {
            SepOrValue::Sep('[') => { depth += 1 },
            SepOrValue::Sep(']') => { depth -= 1 },
            SepOrValue::Value(v) => {
                if depth > 4 && ! exploded {
                    out.pop(); // the '['
                    add_to_next = match flat.get(i+2).unwrap() {Value(v) => Some(v), _ => None};
                    cur = Value(0);
                    i += 3;
                    depth -= 1;
                    exploded = true;
                    if prev_index.is_some() {
                        out[prev_index.unwrap()] = Value(v + match flat.get(prev_index.unwrap()).unwrap() {Value(v) => *v, _ => 0});
                    }
                } else {
                    cur = Value(v + add_to_next.unwrap_or(&0));
                    add_to_next = None;
                }
                prev_index = Some(i);
            },
            _ => (),
        }
        out.push(cur);
        i += 1;
    }

    let s = &flat_to_string(&out);
    parseTree(s)
}

fn explode(node: Link, depth: u32) -> Link {
    //dbg!(&node);
    if node.is_none() {
        return None;
    }
    let n = *node.unwrap();
    if depth < 4 {
        Some(Box::new(Node {
            left: explode(n.left, depth+1),
            right: explode(n.right, depth+1),
            value: n.value,
        }))
    } else {
        if n.value.is_some() {
            Some(Box::new(n.clone()))
        } else {
            Some(Box::new(
                Node {
                    left: None,
                    right: None,
                    value: Some(0),
                }
            ))
        }
    }
}

fn add(first: &str, second: &str) -> String {
    format!("[{},{}]", first, second)
}

fn add2(first: Tree, second: Tree) -> Tree {
    Tree{
        root: Node {
            left: Some(Box::new(first.root)),
            right: Some(Box::new(second.root)),
            value: None,
        }
    }
}

fn to_string(node: Node) -> String {
    if node.value.is_some() {
        return node.value.unwrap().to_string()
    } else {
        let mut out = "[".to_string();
        out += to_string(*node.left.unwrap()).as_str();
        out.push_str( ",");
        out += to_string(*node.right.unwrap()).as_str();
        out += "]";
        out
    }
}

fn to_string2(node: Node) -> String {
    let mut flat = Vec::new();
    flatten(node, &mut flat);
    flat_to_string(&mut flat)
}

fn flat_to_string(flat: &Vec<SepOrValue>) -> String {
    let mut out = String::new();
    for sor in flat {
        match sor {
            SepOrValue::Sep(c) => out.push(*c),
            SepOrValue::Value(v) => out.push_str(&v.to_string()),
        }
    }
    out
}

fn flatten(node: Node, out: &mut Vec<SepOrValue>) {
    if node.value.is_some() {
        out.push(SepOrValue::Value(node.value.unwrap()));
    } else {
        out.push(SepOrValue::Sep('['));
        flatten(*node.left.unwrap(), out);
        out.push(SepOrValue::Sep(','));
        flatten(*node.right.unwrap(), out);
        out.push(SepOrValue::Sep(']'));
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn test_to_string() {
        let num = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        assert_eq!(num, to_string(parseTree(num).root));
        assert_eq!(num, to_string2(parseTree(num).root));
    }

    #[test_case("9" => is eq(9); "num")]
    #[test_case("[9,1]" => is eq(29); "0")]
    #[test_case("[[1,2],[[3,4],5]]" => is eq(143); "1")]
    #[test_case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" => is eq(1384); "2")]
    #[test_case("[[[[1,1],[2,2]],[3,3]],[4,4]]" => is eq(445); "3")]
    #[test_case("[[[[3,0],[5,3]],[4,4]],[5,5]]" => is eq(791); "4")]
    #[test_case("[[[[5,0],[7,4]],[5,5]],[6,6]]" => is eq(1137); "5")]
    #[test_case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" => is eq(3488); "6")]
    fn magnitude(input: &str) -> i32 {
        assert_eq!(mag2(parseTree(input).root), mag(input));
        mag(input)
    }

    #[test_case("[1,2]","[[3,4],5]" => is eq("[[1,2],[[3,4],5]]"); "simple add")]
    fn adding(first: &str, second: &str) -> String {
        let added = add(first, second);
        let added2 = add2(parseTree(first), parseTree(second));

        assert_eq!(added, to_string(added2.root));

        added
    }

    #[test_case("[[[[[9,8],1],2],3],4]","[[[[0,9],2],3],4]" => is eq(true); "first")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]","[7,[6,[5,[7,0]]]]" => is eq(true); "last")]
    #[test_case("[[6,[5,[4,[3,2]]]],1]","[[6,[5,[7,0]]],3]" => is eq(true); "in middle")]
    #[test_case("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]","[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]" => is eq(true); "one at a time")]
    #[test_case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]","[[3,[2,[8,0]]],[9,[5,[7,0]]]]" => is eq(true); "last2")]
    fn exploding(input: &str, out: &str) -> bool {
        assert_eq!(to_string(explodeTree(parseTree(input)).root), out);
        assert_eq!(explodeTree(parseTree(input)), parseTree(out));
        true
    }

    #[test_case("10","[5,5]" => is eq(true); "simple")]
    #[test_case("[[[[0,7],4],[15,[0,13]]],[1,1]]","[[[[0,7],4],[[7,8],[0,13]]],[1,1]]" => is eq(true); "first")]
    #[test_case("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]","[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]" => is eq(true); "second")]
    fn split(input: &str, out: &str) -> bool {
        assert_eq!(splitTree(parseTree(input)), parseTree(out));
        true
    }

    #[test_case("sample1.txt" => is eq(4140); "sample1")]
    #[test_case("input.txt" => is eq(3763); "input")]
    fn part1(input: &str) -> i32 {
        let string = read_to_string(input).unwrap();
        let mut nums = string.lines().map(parseTree);
        let mut sum = nums.next().unwrap();
        for num in nums {
            sum = add2(sum, num);
            let mut changed = true;
            while changed {
                let exploded = explodeTree(sum.clone());
                if exploded == sum {
                    let splitted = splitTree(exploded.clone());
                    if splitted == exploded {
                        changed = false
                    } else {
                        sum = splitted;
                    }
                } else {
                    sum = exploded;
                }
            }
        }
        mag2(sum.root)
    }
}
