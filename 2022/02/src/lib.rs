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
    fn from(first: &str, second: &str) -> anyhow::Result<(RPS, RPS)> {
        Ok(match (first, second) {
            ("A", "X") => (RPS::Rock, RPS::Rock),
            ("A", "Y") => (RPS::Rock, RPS::Paper),
            ("A", "Z") => (RPS::Rock, RPS::Scissors),
            ("B", "X") => (RPS::Paper, RPS::Rock),
            ("B", "Y") => (RPS::Paper, RPS::Paper),
            ("B", "Z") => (RPS::Paper, RPS::Scissors),
            ("C", "X") => (RPS::Scissors, RPS::Rock),
            ("C", "Y") => (RPS::Scissors, RPS::Paper),
            ("C", "Z") => (RPS::Scissors, RPS::Scissors),
            _ => bail!("Unsupported: {first}, {second}"),
        })
    }

    fn from2(first: &str, second: &str) -> anyhow::Result<(RPS, RPS)> {
        let first = match first {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => bail!("Unsupported: {first}"),
        };
        let second = match (first, second) {
            (f, "X") => f.lose(),
            (f, "Y") => f.draw(),
            (f, "Z") => f.win(),
            _ => bail!("Unsupported: {second}"),
        };
        Ok((first, second))
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

    fn lose(&self) -> RPS {
        match *self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
    fn draw(&self) -> RPS {
        *self
    }
    fn win(&self) -> RPS {
        match *self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

fn parse1(row: &str) -> anyhow::Result<(RPS, RPS)> {
    parse(row, false)
}

fn parse2(row: &str) -> anyhow::Result<(RPS, RPS)> {
    parse(row, true)
}

fn parse(row: &str, part2: bool) -> anyhow::Result<(RPS, RPS)> {
    let (first, second) = scan_fmt!(row, "{} {}", String, String)?;
    if part2 {
        RPS::from2(first.as_str(), second.as_str())
    } else {
        RPS::from(first.as_str(), second.as_str())
    }
}

fn read(
    input: &str,
    parse: fn(&str) -> anyhow::Result<(RPS, RPS)>,
) -> anyhow::Result<Vec<(RPS, RPS)>> {
    read_to_string(input)?.lines().map(parse).collect()
}

fn play(input: &str, parse: fn(&str) -> anyhow::Result<(RPS, RPS)>) -> anyhow::Result<i32> {
    Ok(read(input, parse)?
        .into_iter()
        .map(|(other, me)| other.score(me))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps() {
        assert_eq!(RPS::Rock, RPS::from("A", "X").unwrap().0);
        assert_eq!(RPS::Paper, RPS::from("B", "X").unwrap().0);
        assert_eq!(RPS::Scissors, RPS::from("C", "X").unwrap().0);
        assert_eq!(RPS::Rock, RPS::from("A", "X").unwrap().1);
        assert_eq!(RPS::Paper, RPS::from("A", "Y").unwrap().1);
        assert_eq!(RPS::Scissors, RPS::from("A", "Z").unwrap().1);
        assert!(RPS::from("J", "X").is_err());
    }

    #[test]
    fn parsing() {
        assert_eq!((RPS::Rock, RPS::Rock), parse1("A X").unwrap());
        assert_eq!((RPS::Rock, RPS::Scissors), parse2("A X").unwrap());
    }

    #[test]
    fn reading() {
        assert_eq!(
            vec![
                (RPS::Rock, RPS::Paper),
                (RPS::Paper, RPS::Rock),
                (RPS::Scissors, RPS::Scissors),
            ],
            read("sample.txt", parse1).unwrap()
        );
    }

    #[test]
    fn scoring() {
        assert_eq!(8, RPS::Rock.score(RPS::Paper));
    }

    #[test]
    fn playing() {
        assert_eq!(15, play("sample.txt", parse1).unwrap());
        assert_eq!(15422, play("input.txt", parse1).unwrap());
        assert_eq!(15442, play("input.txt", parse2).unwrap());
    }
    #[test]
    fn lose_draw_win() {
        assert_eq!(RPS::Rock, RPS::Paper.lose());
        assert_eq!(RPS::Paper, RPS::Scissors.lose());
        assert_eq!(RPS::Scissors, RPS::Rock.lose());
        assert_eq!(RPS::Paper, RPS::Paper.draw());
        assert_eq!(RPS::Scissors, RPS::Scissors.draw());
        assert_eq!(RPS::Rock, RPS::Rock.draw());
        assert_eq!(RPS::Scissors, RPS::Paper.win());
        assert_eq!(RPS::Rock, RPS::Scissors.win());
        assert_eq!(RPS::Paper, RPS::Rock.win());
    }
}
