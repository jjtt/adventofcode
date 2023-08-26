use std::collections::HashMap;
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
            Value::Literal(literal)
        } else {
            Value::Variable(s.to_string())
        })
    }
}

impl Value {
    fn eval(&self, expressions: &Expressions, cache: &mut Cache) -> isize {
        match self {
            Value::Literal(v) => *v,
            Value::Variable(v) => {
                if let Some(cached) = cache.get(v) {
                    *cached
                } else {
                    let op = expressions.get(v).unwrap();
                    let value = op.eval(expressions, cache);
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
    fn eval(&self, expressions: &Expressions, cache: &mut Cache) -> isize {
        match self {
            Op::Num(v) => v.eval(expressions, cache),
            Op::Add(a, b) => a.eval(expressions, cache) + b.eval(expressions, cache),
            Op::Sub(a, b) => a.eval(expressions, cache) - b.eval(expressions, cache),
            Op::Mul(a, b) => a.eval(expressions, cache) * b.eval(expressions, cache),
            Op::Div(a, b) => a.eval(expressions, cache) / b.eval(expressions, cache),
        }
    }
}

pub fn part1(input: &str) -> isize {
    let input = read_to_string(input).expect("an input file");
    let mut cache = Cache::new();
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

    expressions
        .get("root")
        .expect("a root")
        .eval(&expressions, &mut cache)
}

pub fn part2(input: &str) -> isize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_shouts() {
        assert_eq!(Op::Num(Value::Literal(0)), Op::from_str("0").unwrap());
        assert_eq!(
            Op::Add(Value::Literal(0), Value::Variable("a".to_string())),
            Op::from_str("0 + a").unwrap()
        );
        assert_eq!(
            Op::Sub(Value::Variable("a".to_string()), Value::Literal(1)),
            Op::from_str("a - 1").unwrap()
        );
        assert_eq!(
            Op::Mul(Value::Literal(-1), Value::Literal(1)),
            Op::from_str("-1 * 1").unwrap()
        );
        assert_eq!(
            Op::Div(
                Value::Variable("a".to_string()),
                Value::Variable("b".to_string())
            ),
            Op::from_str("a / b").unwrap()
        );
    }
    #[test]
    fn evaluating() {
        let mut cache = Cache::new();
        let mut expressions = Expressions::new();
        expressions.insert(
            "a".to_string(),
            Op::Add(
                Value::Variable("b".to_string()),
                Value::Variable("b".to_string()),
            ),
        );
        expressions.insert("b".to_string(), Op::Num(Value::Literal(1)));
        assert_eq!(
            2,
            expressions.get("a").unwrap().eval(&expressions, &mut cache)
        );
        assert_eq!(1, cache.len());
        assert!(cache.contains_key("b"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(152, part1("sample.txt"));
    }
}
