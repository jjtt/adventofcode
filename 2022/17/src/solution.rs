use core::fmt;
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::fs::read_to_string;

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
    Jay    = 0b_0010000_0010000_1110000_0000000_isize,
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
            BlockType::Jay => 3,
            BlockType::Vert => 4,
            BlockType::Square => 2,
        }
    }
    fn width(&self) -> u8 {
        match self {
            BlockType::Horiz => 4,
            BlockType::Plus => 3,
            BlockType::Jay => 3,
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
                2 => BlockType::Jay,
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
    fn is_blocked(&self, pile: &Pile) -> bool {
        if self.shifted < 0 || self.shifted as u8 > (7 - self.t.width()) || self.row == 0 {
            return true;
        }

        let block = self.as_pile();
        let overlap = if self.row > pile.top {
            0
        } else {
            pile.top - self.row + 1
        };
        for (i, row) in block
            .iter()
            .take(self.t.height() as usize)
            .rev()
            .take(overlap)
            .enumerate()
        {
            if row & pile.row(self.row + i) > 0 {
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

    fn try_move(self, pile: &Pile, jet: char) -> Block {
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

    #[allow(clippy::unusual_byte_groupings)]
    fn as_pile(&self) -> [u8; 4] {
        let shifted = self.t as u32 >> self.shifted;
        [
            ((shifted & 0b_0000_1111111_0000000_0000000_0000000u32) >> 21) as u8,
            ((shifted & 0b_0000_0000000_1111111_0000000_0000000u32) >> 14) as u8,
            ((shifted & 0b_0000_0000000_0000000_1111111_0000000u32) >> 7) as u8,
            (shifted & 0b_0000_0000000_0000000_0000000_1111111u32) as u8,
        ]
    }
}

struct Pile {
    top: usize,
    pile: VecDeque<u8>,
}

impl fmt::Debug for Pile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pile {{\ntop: {},", self.top)?;
        for r in self.pile.iter().rev() {
            writeln!(f, "{:07b}", r)?;
        }
        Ok(())
    }
}

impl Pile {
    fn new(top: usize) -> Pile {
        let bottom = (0..=top).map(|_| u8::MAX);
        Pile {
            top,
            pile: VecDeque::from_iter(bottom),
        }
    }

    fn add(&mut self, block: Block) -> &Self {
        let as_pile = block.as_pile();

        let block_top = block.row + block.t.height() as usize - 1;
        let top = block_top.min(self.top);

        for (i, row) in as_pile
            .iter()
            .take(block.t.height() as usize)
            .rev()
            .enumerate()
        {
            if block.row + i <= top {
                self.update_row(block.row + i, *row);
            } else {
                self.pile.push_back(*row);
            }
        }
        self.top = self.top.max(block.top());
        self
    }

    pub(crate) fn row(&self, row: usize) -> u8 {
        self.pile[row]
    }
    fn update_row(&mut self, row: usize, update: u8) {
        self.pile[row] |= update;
    }
}

fn drop(count: usize, jets: &str) -> usize {
    let mut jets = jets.chars().cycle();
    let mut source = BlockSource { counter: 0 };
    let mut pile = Pile::new(0);
    while source.counter < count {
        let mut b = source.next(pile.top);
        loop {
            b = b.try_move(&pile, jets.next().expect("Endless jets"));
            let dropped = b.drop();
            if dropped.is_blocked(&pile) {
                pile.add(b);
                break;
            }
            b = dropped;
        }
    }
    pile.top
}

pub fn part1(input: &str) -> usize {
    drop(2022, read_to_string(input).unwrap().trim())
}

pub fn part2(input: &str) -> usize {
    drop(1000000000000, read_to_string(input).unwrap().trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_itself() {
        let mut source = BlockSource { counter: 0 };
        for _ in 0..5 {
            let b = source.next(0);
            assert!(b.is_blocked(Pile::new(3).add(b.clone())))
        }
    }

    #[test]
    fn new_vert_does_not_block_new_plus() {
        let mut source = BlockSource { counter: 0 };
        let vert = source.next(0);
        let plus = source.next(vert.top());
        assert!(!plus.is_blocked(Pile::new(3).add(vert.clone())));
    }

    #[test]
    fn new_is_never_blocked() {
        let mut source = BlockSource { counter: 0 };
        let pile = Pile::new(0);
        for _ in 0..5 {
            let b = source.next(0);
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
        assert!(b.left().is_blocked(&Pile::new(0)));
        assert!(b
            .right()
            .right()
            .right()
            .right()
            .right()
            .right()
            .right()
            .is_blocked(&Pile::new(0)));
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
        assert!(!horiz.is_blocked(Pile::new(0).add(plus.clone())));
        assert!(!plus.is_blocked(Pile::new(0).add(horiz.clone())));
    }

    #[test]
    fn drop_one() {
        assert_eq!(1, drop(1, ">>>>>>>>"));
    }

    #[test]
    fn drop_two() {
        assert_eq!(4, drop(2, ">"));
    }

    #[test]
    fn drop_two_interleaved() {
        assert_eq!(3, drop(2, ">>>><<<<"));
    }

    #[test]
    fn drop_three_interleaved() {
        assert_eq!(4, drop(3, ">>>><<<<>>>>"));
    }

    #[test]
    fn drop_three_interleaved_differently() {
        assert_eq!(4, drop(3, ">>>>>>>>><<<<<<<<"));
    }

    #[test]
    fn drop_four_interleaved() {
        assert_eq!(8, drop(4, ">>>><<<<<<<<<<<<<<<<<"));
    }

    #[test]
    fn drop_two_different_wind() {
        assert_eq!(3, drop(2, ">>>><<<<<"));
    }

    #[test]
    fn add_horiz_to_pile() {
        let mut pile = Pile::new(0);
        let pile = pile.add(Block {
            shifted: 0,
            row: 1,
            t: BlockType::Horiz,
        });

        assert_eq!(pile.top, 1);
        assert_eq!(pile.row(1), 0b01111000);
    }

    #[test]
    fn add_jay_to_pile() {
        let mut pile = Pile::new(0);
        let pile = pile.add(Block {
            shifted: 0,
            row: 1,
            t: BlockType::Jay,
        });

        assert_eq!(pile.top, 3);
        assert_eq!(pile.row(1), 0b01110000);
        assert_eq!(pile.row(2), 0b00010000);
        assert_eq!(pile.row(3), 0b00010000);
    }

    #[test]
    fn jay_on_1_is_blocked_by_horiz_on_1() {
        let mut pile = Pile::new(0);
        let pile = pile.add(Block {
            shifted: 0,
            row: 1,
            t: BlockType::Horiz,
        });

        let mut jay = Block {
            shifted: 2,
            row: 1,
            t: BlockType::Jay,
        };

        assert!(jay.is_blocked(pile));

        jay.shifted = 4;
        assert!(!jay.is_blocked(pile));
    }

    #[test]
    fn horiz_as_pile() {
        let block = Block {
            shifted: 0,
            row: 0,
            t: BlockType::Horiz,
        };

        assert_eq!(block.as_pile(), [0b01111000, 0, 0, 0]);
    }
    #[test]
    fn vert_as_pile() {
        let block = Block {
            shifted: 6,
            row: 0,
            t: BlockType::Vert,
        };

        assert_eq!(
            block.as_pile(),
            [0b00000001, 0b00000001, 0b00000001, 0b00000001]
        );
    }

    #[test]
    fn jay_as_pile() {
        let block = Block {
            shifted: 4,
            row: 0,
            t: BlockType::Jay,
        };

        assert_eq!(
            block.as_pile(),
            [0b00000001, 0b00000001, 0b00000111, 0b00000000]
        );
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
