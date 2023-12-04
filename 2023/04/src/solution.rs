use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::FromStr;

struct Card {
    id: usize,
    winners: HashSet<usize>,
    my: Vec<usize>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, nums) = s.split_once(": ").expect("a card");
        let id = scan_fmt!(id, "Card {d}", usize).expect("a card id");
        let (winners, my) = nums.split_once(" | ").expect("winners and my");
        let winners = winners
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().expect("a winner"))
            .collect();
        let my = my
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().expect("a my"))
            .collect();
        Ok(Card { id, winners, my })
    }
}

impl Card {
    fn worth(self) -> usize {
        let count = self.my.iter().filter(|n| self.winners.contains(n)).count() as u32;
        if count > 0 {
            2usize.pow(count - 1)
        } else {
            0
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).expect("input file");
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Card>().expect("a card"))
        .map(Card::worth)
        .sum()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(15205, part1("input.txt"));
    }
}
