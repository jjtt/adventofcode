use anyhow::bail;
use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum SnafuDigit {
    MinusTwo = -2,
    MinusOne = -1,
    Zero = 0,
    One = 1,
    Two = 2,
}

impl TryFrom<SnafuDigit> for char {
    type Error = ();

    fn try_from(value: SnafuDigit) -> Result<Self, Self::Error> {
        match value {
            SnafuDigit::MinusTwo => Ok('='),
            SnafuDigit::MinusOne => Ok('-'),
            SnafuDigit::Zero => Ok('0'),
            SnafuDigit::One => Ok('1'),
            SnafuDigit::Two => Ok('2'),
        }
    }
}

impl TryFrom<char> for SnafuDigit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '=' => Ok(SnafuDigit::MinusTwo),
            '-' => Ok(SnafuDigit::MinusOne),
            '0' => Ok(SnafuDigit::Zero),
            '1' => Ok(SnafuDigit::One),
            '2' => Ok(SnafuDigit::Two),
            _ => bail!("Invalid digit: {}", value),
        }
    }
}

impl TryFrom<i32> for SnafuDigit {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -2 => Ok(SnafuDigit::MinusTwo),
            -1 => Ok(SnafuDigit::MinusOne),
            0 => Ok(SnafuDigit::Zero),
            1 => Ok(SnafuDigit::One),
            2 => Ok(SnafuDigit::Two),
            _ => bail!("Invalid digit: {}", value),
        }
    }
}

impl Add<SnafuDigit> for SnafuDigit {
    type Output = (SnafuDigit, i32);

    fn add(self, rhs: SnafuDigit) -> Self::Output {
        let rhs = rhs as i32;
        self.add(rhs)
    }
}

impl Add<i32> for SnafuDigit {
    type Output = (SnafuDigit, i32);

    fn add(self, rhs: i32) -> Self::Output {
        let sum = self as i32 + rhs;
        if sum + 2 < 0 {
            let carry = (sum + 2 - 5) / 5;
            let sum = (sum + 2 + 5) % 5 - 2;
            (SnafuDigit::try_from(sum).unwrap(), carry)
        } else {
            let carry = (sum + 2) / 5;
            let sum = (sum + 2) % 5 - 2;
            (SnafuDigit::try_from(sum).unwrap(), carry)
        }
    }
}

fn sum(a: &str, b: &str) -> String {
    let len = a.len().max(b.len());
    let mut result = String::new();
    let mut carry = 0;
    for (a, b) in a
        .chars()
        .rev()
        .pad_using(len, |_| '0')
        .zip(b.chars().rev().pad_using(len, |_| '0'))
    {
        let a = SnafuDigit::try_from(a).unwrap();
        let b = SnafuDigit::try_from(b).unwrap();
        let (ab, c1) = a + b;
        let (sum, c2) = ab + carry;
        result.push(char::try_from(sum).unwrap());
        carry = c1 + c2;
    }
    if carry > 0 {
        result.push(char::try_from(SnafuDigit::try_from(carry).unwrap()).unwrap());
    }
    result.chars().rev().collect()
}

pub fn part1(input: &str) -> String {
    let input = read_to_string(input).expect("input");
    input
        .lines()
        .fold("0".to_string(), |acc, line| sum(acc.as_str(), line))
}

pub fn part2(_: &str) -> String {
    "Merry X-mas!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0", "0" => "0"; "0+0=0")]
    #[test_case("0", "1" => "1"; "0+1=1")]
    #[test_case("-", "0" => "-"; "m1+0=m1")]
    #[test_case("-", "-" => "="; "m1+m1=m2")]
    #[test_case("=", "0" => "="; "m2+0=m2")]
    #[test_case("1", "1" => "2"; "1+1=2")]
    #[test_case("2", "1" => "1="; "2+1=3")]
    #[test_case("2", "2" => "1-"; "2+2=4")]
    #[test_case("2", "1=" => "10"; "2+3=5")]
    #[test_case("1-", "2" => "11"; "4+2=6")]
    #[test_case("1=", "1=" => "11"; "3+3=6")]
    #[test_case("1-", "1=" => "12"; "4+3=7")]
    #[test_case("1=", "1-" => "12"; "3+4=7")]
    #[test_case("1-", "1-" => "2="; "4+4=8")]
    fn simple_sums(a: &str, b: &str) -> String {
        sum(a, b)
    }

    #[test]
    fn part1_sample() {
        assert_eq!("2=-1=0", part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!("20-1-0=-2=-2220=0011", part1("input.txt"));
    }
}
