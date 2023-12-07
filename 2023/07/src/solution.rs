use itertools::Itertools;
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

type Hand = [Card; 5];

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
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap()
}

#[derive(PartialEq, Eq)]
struct EvaluatedHand {
    hand: Hand,
    value: usize,
}

impl EvaluatedHand {
    pub fn from(hand: Hand) -> EvaluatedHand {
        let grouped = hand.iter().sorted().group_by(|card| *card);
        let sets = grouped
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .rev()
            .collect::<Vec<usize>>();
        let value = match sets.len() {
            1 => 6, // five of a kind
            2 => {
                if sets[0] == 4 {
                    5 // four of a kind
                } else {
                    4 // full house
                }
            }
            3 => {
                if sets[0] == 3 {
                    3 // three of a kind
                } else {
                    2 // two pair
                }
            }
            4 => 1, // one pair
            5 => 0, // high card
            _ => panic!("Invalid hand: {:?}", hand),
        };

        EvaluatedHand { hand, value }
    }
    pub fn from_wild_jacks(hand: Hand) -> EvaluatedHand {
        let hand: Hand = hand
            .into_iter()
            .map(|c| match c {
                Card::Jack => Card::Joker,
                c => c,
            })
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();

        let jokers = hand.iter().filter(|card| **card == Card::Joker).count();
        let filtered_hand = hand
            .iter()
            .filter(|card| **card != Card::Joker)
            .copied()
            .collect::<Vec<Card>>();
        let grouped = filtered_hand.iter().sorted().group_by(|card| *card);
        let sets = grouped
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .rev()
            .collect::<Vec<usize>>();
        let value = match (sets.len(), jokers) {
            (1, _) => 6, // five of a kind
            (2, 1) => {
                if sets[0] == 3 {
                    5 // four of a kind
                } else {
                    4 // full house
                }
            }
            (2, 2) => 5, // four of a kind
            (2, 3) => 5, // four of a kind
            (3, 1) => 3, // three of a kind
            (3, 2) => 1, // one pair
            (4, 1) => 1, // one pair
            _ => EvaluatedHand::from(hand).value,
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
            let (hand, bid) = line.split_once(" ").expect("a valid line");
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

    #[test_case("AAAAA" => 6)]
    #[test_case("AA8AA" => 5)]
    #[test_case("23332" => 4)]
    #[test_case("TTT98" => 3)]
    #[test_case("23432" => 2)]
    #[test_case("A23A4" => 1)]
    #[test_case("23456" => 0)]
    fn evaluated_hand_value(hand: &str) -> usize {
        EvaluatedHand::from(parse_hand(hand)).value
    }

    #[test_case("AAAAA" => 6)]
    #[test_case("AA8AA" => 5)]
    #[test_case("23332" => 4)]
    #[test_case("TTT98" => 3)]
    #[test_case("23432" => 2)]
    #[test_case("A23A4" => 1)]
    #[test_case("23456" => 0)]
    #[test_case("32T3K" => 1)]
    #[test_case("KK677" => 2)]
    #[test_case("T55J5" => 5)]
    #[test_case("KTJJT" => 5)]
    #[test_case("QQQJA" => 5)]
    #[test_case("JJJJJ" => 6)]
    #[test_case("JJJJA" => 6)]
    #[test_case("JJJAA" => 6)]
    #[test_case("JJAAA" => 6)]
    #[test_case("JAAAA" => 6)]
    #[test_case("JJJA2" => 5)]
    #[test_case("JJAA2" => 5)]
    #[test_case("JAAA2" => 5)]
    #[test_case("AAAA2" => 5)]
    #[test_case("JAA22" => 4)]
    #[test_case("AAA22" => 4)]
    #[test_case("J2345" => 1)]
    #[test_case("JAA23" => 3)]
    fn evaluated_joker_hand_value(hand: &str) -> usize {
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
        assert_eq!(0, part2("input.txt"));
    }
}
