use scan_fmt::scan_fmt;
use std::fmt;
use std::fmt::Formatter;
use std::fs::read_to_string;

struct Board {
    stacks: Vec<String>,
}

impl Board {
    pub(crate) fn make_some(&mut self, moves: Moves) {
        for (count, from, to) in moves.moves {
            for _ in 0..count {
                let what = self.stacks.get_mut(from - 1).unwrap().pop().unwrap();
                self.stacks.get_mut(to - 1).unwrap().push(what);
            }
        }
    }

    pub(crate) fn make_some_modern(&mut self, moves: Moves) {
        for (count, from, to) in moves.moves {
            let mut crane = String::new();
            for _ in 0..count {
                let what = self.stacks.get_mut(from - 1).unwrap().pop().unwrap();
                crane.insert(0, what);
            }
            self.stacks.get_mut(to - 1).unwrap().push_str(&crane);
        }
    }
}

impl Board {
    pub(crate) fn result(&self) -> String {
        let mut result = String::new();

        for stack in self.stacks.iter() {
            result.push(stack.chars().last().unwrap());
        }

        result
    }
}

impl Board {
    pub(crate) fn add(&mut self, input_line: &str) {
        if !input_line.contains('[') {
            return;
        }

        let new = self.stacks.is_empty();
        for index in 0..input_line.len() / 4 + 1 {
            if new {
                self.stacks.push(String::new());
            };

            let x = input_line.chars().nth(index * 4 + 1).unwrap();
            if x != ' ' {
                let stack = self.stacks.get_mut(index).unwrap();
                stack.insert(0, x);
            }
        }
    }
}

impl Board {
    pub(crate) fn new() -> Board {
        Board { stacks: vec![] }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_depth = self.stacks.iter().map(String::len).max().unwrap_or(0);

        for row in (0..max_depth).rev() {
            let mut first = true;
            for stack in self.stacks.iter() {
                let nothing = String::from("   ");
                let item = stack
                    .get(row..=row)
                    .map(|i| format!("[{i}]"))
                    .unwrap_or(nothing);
                write!(f, "{}{}", if first { "" } else { " " }, item,)?;
                first = false;
            }
            writeln!(f)?;
        }

        for (stack, _) in self.stacks.iter().enumerate() {
            write!(f, "{} {} ", if stack == 0 { "" } else { " " }, stack + 1)?;
        }
        writeln!(f)
    }
}

struct Moves {
    moves: Vec<(usize, usize, usize)>,
}

impl Moves {
    pub(crate) fn add(&mut self, input_line: &str) {
        let m = scan_fmt!(input_line, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        self.moves.push(m);
    }
}

impl Moves {
    pub(crate) fn new() -> Moves {
        Moves { moves: vec![] }
    }
}

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (count, from, to) in self.moves.iter() {
            writeln!(f, "move {} from {} to {}", count, from, to)?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> (Board, Moves) {
    let mut board = Board::new();
    let mut moves = Moves::new();
    let mut parsing_board = true;
    for line in read_to_string(input).unwrap().lines() {
        if line.is_empty() {
            parsing_board = false;
        } else if parsing_board {
            board.add(line);
        } else {
            moves.add(line);
        }
    }

    (board, moves)
}

pub fn part1(input: &str) -> String {
    let (mut board, moves) = parse(input);

    board.make_some(moves);

    board.result()
}

pub fn part2(input: &str) -> String {
    let (mut board, moves) = parse(input);

    board.make_some_modern(moves);

    board.result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displaying_a_board() {
        let b = Board {
            stacks: vec![String::from("ZN"), String::from("MCD"), String::from("P")],
        };

        println!("{b}");
    }

    #[test]
    fn displaying_moves() {
        let m = Moves {
            moves: vec![(1, 2, 3)],
        };

        println!("{m}");
    }

    #[test]
    fn parsing() {
        let (board, moves) = parse("sample.txt");

        assert_eq!(
            read_to_string("sample.txt").unwrap(),
            format!("{board}\n{moves}")
        );
    }

    #[test]
    fn result_from_board() {
        let (board, _) = parse("sample.txt");
        assert_eq!("NDP", board.result());
    }

    #[test]
    fn part1_sample() {
        assert_eq!("CMZ", part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!("MCD", part2("sample.txt"));
    }
}
