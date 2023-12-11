use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

type Hand = Vec<Card>;

fn parse_hand(s: &str) -> Hand {
    s.chars()
        .map(|c| match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        })
        .collect()
}

#[derive(PartialEq, Eq)]
struct EvaluatedHand {
    hand: Hand,
    value: (usize, usize, usize, usize, usize),
}

impl EvaluatedHand {
    pub fn from(hand: Hand) -> EvaluatedHand {
        let groups = hand
            .iter()
            .sorted()
            .group_by(|card| *card)
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .group_by(|group_size| *group_size)
            .into_iter()
            .map(|(group_size, group)| (group_size, group.count()))
            .collect::<HashMap<_, _>>();
        let value = (1..=5)
            .rev()
            .map(|i| {
                if let Some(count) = groups.get(&i) {
                    *count
                } else {
                    0
                }
            })
            .collect_tuple()
            .unwrap();

        EvaluatedHand { hand, value }
    }
    pub fn from_wild_jacks(hand: Hand) -> EvaluatedHand {
        let hand: Hand = hand
            .into_iter()
            .map(|c| match c {
                Card::Jack => Card::Joker,
                c => c,
            })
            .collect();

        let jokers = hand.iter().filter(|card| **card == Card::Joker).count();
        let filtered_hand = hand
            .iter()
            .filter(|card| **card != Card::Joker)
            .copied()
            .collect::<Vec<Card>>();

        let (fives, fours, threes, pairs, singles) = EvaluatedHand::from(filtered_hand).value;

        let value = match jokers {
            0 => (fives, fours, threes, pairs, singles),
            1 if fours == 1 => (1, 0, 0, 0, 0),
            1 if threes == 1 => (0, 1, 0, 0, 1),
            1 if pairs > 0 => (0, 0, 1, pairs - 1, singles),
            1 => (0, 0, 0, 1, 3),
            2 if threes == 1 => (1, 0, 0, 0, 0),
            2 if pairs == 1 => (0, 1, 0, 0, 1),
            2 => (0, 0, 1, 0, 2),
            3 if pairs == 1 => (1, 0, 0, 0, 0),
            3 => (0, 1, 0, 0, 1),
            4 => (1, 0, 0, 0, 0),
            5 => (1, 0, 0, 0, 0),
            _ => panic!("Invalid number of jokers: {}", jokers),
        };

        EvaluatedHand { hand, value }
    }
}

impl PartialOrd for EvaluatedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let value_cmp = self.value.cmp(&other.value);
        match value_cmp {
            std::cmp::Ordering::Equal => self
                .hand
                .iter()
                .zip(other.hand.iter())
                .map(|(self_card, other_card)| self_card.cmp(other_card))
                .find(|&cmp| cmp != std::cmp::Ordering::Equal)
                .or(Some(std::cmp::Ordering::Equal)),
            _ => Some(value_cmp),
        }
    }
}

impl Ord for EvaluatedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn evaluate(input: &str, jacks_wild: bool) -> usize {
    let input = read_to_string(input).unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("a valid line");
            (
                if jacks_wild {
                    EvaluatedHand::from_wild_jacks(parse_hand(hand))
                } else {
                    EvaluatedHand::from(parse_hand(hand))
                },
                bid.parse::<usize>().expect("a valid bid"),
            )
        })
        .sorted()
        .enumerate()
        .map(|(i, (_hand, bid))| (i + 1) * bid)
        .sum()
}
pub fn part1(input: &str) -> usize {
    evaluate(input, false)
}
pub fn part2(input: &str) -> usize {
    evaluate(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("AAAAA" => (1, 0, 0, 0, 0))]
    #[test_case("AA8AA" => (0, 1, 0, 0, 1))]
    #[test_case("23332" => (0, 0, 1, 1, 0))]
    #[test_case("TTT98" => (0, 0, 1, 0, 2))]
    #[test_case("23432" => (0, 0, 0, 2, 1))]
    #[test_case("A23A4" => (0, 0, 0, 1, 3))]
    #[test_case("23456" => (0, 0, 0, 0, 5))]
    fn evaluated_hand_value(hand: &str) -> (usize, usize, usize, usize, usize) {
        EvaluatedHand::from(parse_hand(hand)).value
    }

    #[test_case("JJJJJ" => (1, 0, 0, 0, 0))]
    #[test_case("JAA23" => (0, 0, 1, 0, 2))]
    fn evaluated_joker_hand_value(hand: &str) -> (usize, usize, usize, usize, usize) {
        EvaluatedHand::from_wild_jacks(parse_hand(hand)).value
    }

    #[test]
    fn part1_sample() {
        assert_eq!(6440, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(248836197, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(5905, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(251195607, part2("input.txt"));
    }
}
