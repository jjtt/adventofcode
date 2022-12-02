use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

#[derive(PartialEq, Debug, Copy, Clone)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn from(input: &str) -> anyhow::Result<RPS> {
        Ok(match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => bail!("Unsupported: {input}"),
        })
    }

    fn score(&self, my: RPS) -> i32 {
        my as i32
            + match (*self, my) {
                (RPS::Rock, RPS::Paper) => 6,
                (RPS::Paper, RPS::Scissors) => 6,
                (RPS::Scissors, RPS::Rock) => 6,
                (RPS::Rock, RPS::Scissors) => 0,
                (RPS::Paper, RPS::Rock) => 0,
                (RPS::Scissors, RPS::Paper) => 0,
                _ => 3,
            }
    }
}

fn parse(row: &str) -> anyhow::Result<(RPS, RPS)> {
    let (first, second) = scan_fmt!(row, "{} {}", String, String)?;
    Ok((RPS::from(first.as_str())?, RPS::from(second.as_str())?))
}

fn read(input: &str) -> anyhow::Result<Vec<(RPS, RPS)>> {
    read_to_string(input)?.lines().map(parse).collect()
}

fn play(input: &str) -> anyhow::Result<i32> {
    Ok(read(input)?
        .into_iter()
        .map(|(other, me)| other.score(me))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps() {
        assert_eq!(RPS::Rock, RPS::from("A").unwrap());
        assert_eq!(RPS::Paper, RPS::from("B").unwrap());
        assert_eq!(RPS::Scissors, RPS::from("C").unwrap());
        assert_eq!(RPS::Rock, RPS::from("X").unwrap());
        assert_eq!(RPS::Paper, RPS::from("Y").unwrap());
        assert_eq!(RPS::Scissors, RPS::from("Z").unwrap());
        assert!(RPS::from("J").is_err());
    }

    #[test]
    fn parsing() {
        assert_eq!((RPS::Rock, RPS::Rock), parse("A X").unwrap());
    }

    #[test]
    fn reading() {
        assert_eq!(
            vec![
                (RPS::Rock, RPS::Paper),
                (RPS::Paper, RPS::Rock),
                (RPS::Scissors, RPS::Scissors),
            ],
            read("sample.txt").unwrap()
        );
    }

    #[test]
    fn scoring() {
        assert_eq!(8, RPS::Rock.score(RPS::Paper));
    }

    #[test]
    fn playing() {
        assert_eq!(15, play("sample.txt").unwrap());
        assert_eq!(15422, play("input.txt").unwrap());
    }
}
