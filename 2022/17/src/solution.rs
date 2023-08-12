use std::fs::read_to_string;
use std::iter::Cycle;
use std::str::Chars;

#[derive(Clone, Debug)]
struct Block {
    shifted: i8,
    row: usize,
    t: BlockType,
}

#[derive(Clone, Copy, Debug)]
enum BlockType {
    #[rustfmt::skip]
    Horiz  = 0b_1111000_0000000_0000000_0000000_isize,
    #[rustfmt::skip]
    Plus   = 0b_0100000_1110000_0100000_0000000_isize,
    #[rustfmt::skip]
    Jey    = 0b_0010000_0010000_1110000_0000000_isize,
    #[rustfmt::skip]
    Vert   = 0b_1000000_1000000_1000000_1000000_isize,
    #[rustfmt::skip]
    Square = 0b_1100000_1100000_0000000_0000000_isize,
}

impl BlockType {
    fn height(&self) -> u8 {
        match self {
            BlockType::Horiz => 1,
            BlockType::Plus => 3,
            BlockType::Jey => 3,
            BlockType::Vert => 4,
            BlockType::Square => 2,
        }
    }
    fn width(&self) -> u8 {
        match self {
            BlockType::Horiz => 4,
            BlockType::Plus => 3,
            BlockType::Jey => 3,
            BlockType::Vert => 1,
            BlockType::Square => 2,
        }
    }
}

struct BlockSource {
    counter: usize,
}

impl BlockSource {
    fn next(&mut self, top: usize) -> Block {
        self.counter += 1;
        Block {
            shifted: 2,
            row: 4 + top,
            t: match (self.counter - 1) % 5 {
                0 => BlockType::Horiz,
                1 => BlockType::Plus,
                2 => BlockType::Jey,
                3 => BlockType::Vert,
                4 => BlockType::Square,
                _ => {
                    panic!("Unsupported block type")
                }
            },
        }
    }
}

impl Block {
    fn is_blocked_by(&self, other: &Block, height_diff: u8) -> bool {
        if height_diff >= self.t.height() {
            false
        } else {
            let this_shifted = self.t as isize >> self.shifted;
            let other_shifted = other.t as isize >> other.shifted;
            let other_dropped = other_shifted >> (height_diff * 7);
            (this_shifted & other_dropped) > 0
        }
    }
    fn is_blocked(&self, pile: &[Block]) -> bool {
        if self.shifted < 0 || self.shifted as u8 > (7 - self.t.width()) || self.row == 0 {
            return true;
        }

        for other in pile.iter().rev() {
            let self_top = self.row + self.t.height() as usize - 1;
            let other_top = other.row + other.t.height() as usize - 1;
            if self_top.abs_diff(other_top) < 5
                && if self_top > other_top {
                    let diff = self_top - other_top;
                    self.is_blocked_by(other, diff.try_into().expect("small number"))
                } else {
                    let diff = other_top - self_top;
                    other.is_blocked_by(self, diff.try_into().expect("small number"))
                }
            {
                return true;
            }
        }
        false
    }

    fn top(&self) -> usize {
        self.row + self.t.height() as usize - 1
    }

    fn right(&self) -> Block {
        Block {
            shifted: self.shifted + 1,
            row: self.row,
            t: self.t,
        }
    }

    fn left(&self) -> Block {
        Block {
            shifted: self.shifted - 1,
            row: self.row,
            t: self.t,
        }
    }

    fn drop(&self) -> Block {
        Block {
            shifted: self.shifted,
            row: self.row - 1,
            t: self.t,
        }
    }

    fn try_move(self, pile: &[Block], jet: char) -> Block {
        let moved = match jet {
            '>' => self.right(),
            '<' => self.left(),
            _ => panic!("Unsupported jets"),
        };
        if moved.is_blocked(pile) {
            self
        } else {
            moved
        }
    }
}

fn drop(count: usize, mut jets: Cycle<Chars>) -> usize {
    let mut source = BlockSource { counter: 0 };
    let mut top = 0;
    let mut pile = vec![];
    while source.counter < count {
        let mut b = source.next(top);
        loop {
            b = b.try_move(&pile, jets.next().expect("Endless jets"));
            let dropped = b.drop();
            if dropped.is_blocked(&pile) {
                top = top.max(b.top());
                pile.push(b);
                break;
            }
            b = dropped;
        }
    }
    top
}

pub fn part1(input: &str) -> usize {
    drop(2022, read_to_string(input).unwrap().trim().chars().cycle())
}

pub fn part2(input: &str) -> usize {
    drop(
        1000000000000,
        read_to_string(input).unwrap().trim().chars().cycle(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_itself() {
        let mut source = BlockSource { counter: 0 };
        for _ in 0..5 {
            let b = source.next(0);
            assert!(b.is_blocked(&vec![b.clone()]))
        }
    }

    #[test]
    fn new_vert_does_not_block_new_plus() {
        let mut source = BlockSource { counter: 0 };
        let vert = source.next(0);
        let plus = source.next(vert.top());
        assert!(!plus.is_blocked(&vec![vert.clone()]));
        assert!(!vert.is_blocked(&vec![plus.clone()]));
    }

    #[test]
    fn new_is_never_blocked() {
        let mut source = BlockSource { counter: 0 };
        let mut top = 0;
        let mut pile = vec![];
        let mut b = source.next(top);
        for _ in 0..5 {
            top = b.top();
            pile.push(b);
            b = source.next(top);
            assert!(!b.is_blocked(&pile));
        }
    }

    #[test]
    fn block_outside() {
        let b = Block {
            shifted: 0,
            row: 0,
            t: BlockType::Horiz,
        };
        assert!(b.left().is_blocked(&vec![]));
        assert!(b
            .right()
            .right()
            .right()
            .right()
            .right()
            .right()
            .right()
            .is_blocked(&vec![]));
    }

    #[test]
    fn horiz_and_plus_fit_on_the_same_bottom_row() {
        let horiz = Block {
            shifted: 3,
            row: 1,
            t: BlockType::Horiz,
        };
        let plus = Block {
            shifted: 0,
            row: 1,
            t: BlockType::Plus,
        };
        assert!(!horiz.is_blocked(&[plus.clone()]));
        assert!(!plus.is_blocked(&[horiz.clone()]));
    }

    #[test]
    fn drop_one() {
        assert_eq!(1, drop(1, ">>>>>>>>".chars().cycle()));
    }

    #[test]
    fn drop_two() {
        assert_eq!(4, drop(2, ">".chars().cycle()));
    }

    #[test]
    fn drop_two_different_wind() {
        assert_eq!(3, drop(2, ">>>><<<<<".chars().cycle()));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(3068, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(1514285714288, part2("sample.txt"));
    }
}
