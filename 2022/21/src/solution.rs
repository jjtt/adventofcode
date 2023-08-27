use crate::solution::Value::{Literal, Variable};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

type Expressions = HashMap<String, Op>;
type Cache = HashMap<String, isize>;

#[derive(Debug, PartialEq)]
enum Value {
    Literal(isize),
    Variable(String),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(literal) = isize::from_str(s) {
            Literal(literal)
        } else {
            Variable(s.to_string())
        })
    }
}

impl Value {
    fn eval(
        &self,
        expressions: &Expressions,
        cache: &mut Cache,
        inverted: &mut HashSet<String>,
    ) -> isize {
        match self {
            Literal(v) => *v,
            Variable(v) => {
                if v.eq("root") && expressions.contains_key("ROOT") {
                    0
                } else if let Some(cached) = cache.get(v) {
                    *cached
                } else if !inverted.contains(v) && expressions.contains_key(v) {
                    let op = expressions.get(v).unwrap();
                    let value = op.eval(expressions, cache, inverted);
                    cache.insert(v.clone(), value);
                    value
                } else {
                    let mut filtered = expressions.iter().filter(|(_key, op)| match op {
                        Op::Add(Variable(a), Variable(b)) => a.eq(v) || b.eq(v),
                        Op::Sub(Variable(a), Variable(b)) => a.eq(v) || b.eq(v),
                        Op::Mul(Variable(a), Variable(b)) => a.eq(v) || b.eq(v),
                        Op::Div(Variable(a), Variable(b)) => a.eq(v) || b.eq(v),
                        _ => false,
                    });
                    let (old_key, old_op) = filtered
                        .next()
                        .unwrap_or_else(|| panic!("One and only monkey listening for {}", v));
                    assert!(filtered.next().is_none());

                    let inv = old_op.invert(v, old_key);
                    inverted.insert(old_key.clone());
                    let value = inv.eval(expressions, cache, inverted);
                    cache.insert(v.clone(), value);
                    value
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Num(Value),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.contains(" + ") {
            let (a, b) = s.split_once(" + ").unwrap();
            Op::Add(Value::from_str(a).unwrap(), Value::from_str(b).unwrap())
        } else if s.contains(" - ") {
            let (a, b) = s.split_once(" - ").unwrap();
            Op::Sub(Value::from_str(a).unwrap(), Value::from_str(b).unwrap())
        } else if s.contains(" * ") {
            let (a, b) = s.split_once(" * ").unwrap();
            Op::Mul(Value::from_str(a).unwrap(), Value::from_str(b).unwrap())
        } else if s.contains(" / ") {
            let (a, b) = s.split_once(" / ").unwrap();
            Op::Div(Value::from_str(a).unwrap(), Value::from_str(b).unwrap())
        } else {
            Op::Num(Value::from_str(s).unwrap())
        })
    }
}

impl Op {
    fn eval(
        &self,
        expressions: &Expressions,
        cache: &mut Cache,
        inverted: &mut HashSet<String>,
    ) -> isize {
        match self {
            Op::Num(v) => v.eval(expressions, cache, inverted),
            Op::Add(a, b) => {
                a.eval(expressions, cache, inverted) + b.eval(expressions, cache, inverted)
            }
            Op::Sub(a, b) => {
                a.eval(expressions, cache, inverted) - b.eval(expressions, cache, inverted)
            }
            Op::Mul(a, b) => {
                a.eval(expressions, cache, inverted) * b.eval(expressions, cache, inverted)
            }
            Op::Div(a, b) => {
                a.eval(expressions, cache, inverted) / b.eval(expressions, cache, inverted)
            }
        }
    }

    pub(crate) fn invert(&self, new_key: &str, old_key: &str) -> Op {
        match self {
            Op::Add(Variable(a), Variable(b)) if b.eq(new_key) => {
                Op::Sub(Variable(old_key.to_string()), Variable(a.clone()))
            }
            Op::Add(Variable(a), Variable(b)) if a.eq(new_key) => {
                Op::Sub(Variable(old_key.to_string()), Variable(b.clone()))
            }
            Op::Sub(Variable(a), Variable(b)) if a.eq(new_key) => {
                Op::Add(Variable(old_key.to_string()), Variable(b.clone()))
            }
            Op::Sub(Variable(a), Variable(b)) if b.eq(new_key) => {
                Op::Add(Variable(old_key.to_string()), Variable(a.clone()))
            }
            Op::Mul(Variable(a), Variable(b)) if b.eq(new_key) => {
                Op::Div(Variable(old_key.to_string()), Variable(a.clone()))
            }
            Op::Mul(Variable(a), Variable(b)) if a.eq(new_key) => {
                Op::Div(Variable(old_key.to_string()), Variable(b.clone()))
            }
            Op::Div(Variable(a), Variable(b)) if a.eq(new_key) => {
                Op::Mul(Variable(old_key.to_string()), Variable(b.clone()))
            }
            _ => todo!("{}: {:?}", new_key, self),
        }
    }
}

fn parse_expressions(input: &str) -> (Expressions, Cache) {
    let input = read_to_string(input).expect("an input file");
    let expressions = input
        .lines()
        .filter_map(|l| l.split_once(": "))
        .map(|(monkey, shout)| {
            (
                monkey.to_string(),
                Op::from_str(shout).expect("a valid shout"),
            )
        })
        .collect::<Expressions>();
    let cache = expressions
        .iter()
        .filter_map(|(key, op)| {
            if let Op::Num(Literal(v)) = op {
                Some((key.clone(), *v))
            } else {
                None
            }
        })
        .collect();
    (expressions, cache)
}

pub fn part1(input: &str) -> isize {
    let (expressions, mut cache) = parse_expressions(input);

    expressions
        .get("root")
        .expect("a root")
        .eval(&expressions, &mut cache, &mut HashSet::new())
}

pub fn part2(input: &str) -> isize {
    let (mut expressions, mut cache) = parse_expressions(input);

    if let Op::Add(a, b) = expressions.remove("root").expect("a root") {
        expressions.insert("root".to_string(), Op::Sub(a, b));
        //expressions.insert("ROOT".to_string(), Op::Num(Literal(0)));
    } else {
        panic!("Root should be an addition of two variables");
    }

    cache.remove("humn");
    for i in 0..9_993_963_640_759 {
        expressions.insert("humn".to_string(), Op::Num(Literal(i)));
        let value = expressions.get("root").expect("a root").eval(
            &expressions,
            &mut cache.clone(),
            &mut HashSet::new(),
        );
        if value == 0 {
            return i;
        }
    }
    panic!("Not found :(")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_shouts() {
        assert_eq!(Op::Num(Literal(0)), Op::from_str("0").unwrap());
        assert_eq!(
            Op::Add(Literal(0), Variable("a".to_string())),
            Op::from_str("0 + a").unwrap()
        );
        assert_eq!(
            Op::Sub(Variable("a".to_string()), Literal(1)),
            Op::from_str("a - 1").unwrap()
        );
        assert_eq!(
            Op::Mul(Literal(-1), Literal(1)),
            Op::from_str("-1 * 1").unwrap()
        );
        assert_eq!(
            Op::Div(Variable("a".to_string()), Variable("b".to_string())),
            Op::from_str("a / b").unwrap()
        );
    }
    #[test]
    fn evaluating() {
        let mut cache = Cache::new();
        let mut expressions = Expressions::new();
        expressions.insert(
            "a".to_string(),
            Op::Add(Variable("b".to_string()), Variable("b".to_string())),
        );
        expressions.insert("b".to_string(), Op::Num(Literal(1)));
        assert_eq!(
            2,
            expressions
                .get("a")
                .unwrap()
                .eval(&expressions, &mut cache, &mut HashSet::new())
        );
        assert_eq!(1, cache.len());
        assert!(cache.contains_key("b"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(152, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(301, part2("sample.txt"));
    }
}
