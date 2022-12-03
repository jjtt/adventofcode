use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn from(first: &str, second: &str) -> anyhow::Result<(RPS, RPS)> {
        let first = match first {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => bail!("Unsupported: {first}"),
        };
        let second = match second {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => bail!("Unsupported: {second}"),
        };
        Ok((first, second))
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
            + if self.win() == my {
                6
            } else if self.draw() == my {
                3
            } else {
                0
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

pub fn parse1(row: &str) -> anyhow::Result<(RPS, RPS)> {
    parse(row, RPS::from)
}

pub fn parse2(row: &str) -> anyhow::Result<(RPS, RPS)> {
    parse(row, RPS::from2)
}

fn parse<F>(row: &str, from: F) -> anyhow::Result<(RPS, RPS)>
where
    F: Fn(&str, &str) -> anyhow::Result<(RPS, RPS)>,
{
    let (first, second) = scan_fmt!(row, "{} {}", String, String)?;
    from(first.as_str(), second.as_str())
}

fn read<F>(input: &str, parse: F) -> anyhow::Result<Vec<(RPS, RPS)>>
where
    F: Fn(&str) -> anyhow::Result<(RPS, RPS)>,
{
    read_to_string(input)?.lines().map(parse).collect()
}

pub fn play(input: &str, parse: fn(&str) -> anyhow::Result<(RPS, RPS)>) -> anyhow::Result<i32> {
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
