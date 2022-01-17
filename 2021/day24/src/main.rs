use std::fs::read_to_string;

enum Instruction {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
}

struct Op {
    instruction: Instruction,
    a: i32,
    b: i32,
    b_literal: Option<i32>,
}

impl Op {
    fn parse(s: &str) -> Op {
        todo!()
    }
}

struct Program {
    ops: Vec<Op>,
    pc: i32,
}

impl Program {
    fn parse_program(s: String) -> Program {
        Program {
            ops: s.lines().map(Op::parse).collect(),
            pc: 0,
        }
    }

    fn run(&self, input: Vec<i32>) -> (i32, i32, i32, i32) {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let program = Program::parse_program(read_to_string("sample1.txt").unwrap());

        assert_eq!(-1, program.run(vec![1]).1);
        assert_eq!(1, program.run(vec![-1]).1);
        assert_eq!(-42, program.run(vec![42]).1);
    }

    #[test]
    fn sample2() {
        let program = Program::parse_program(read_to_string("sample2.txt").unwrap());

        assert_eq!(1, program.run(vec![1, 3]).1);
        assert_eq!(1, program.run(vec![2, 6]).1);
        assert_eq!(1, program.run(vec![-2, -6]).1);
        assert_eq!(0, program.run(vec![1, 1]).1);
    }

    #[test]
    fn sample3() {
        let program = Program::parse_program(read_to_string("sample3.txt").unwrap());

        assert_eq!((0, 0, 0, 0), program.run(vec![0]));
        assert_eq!((0, 0, 0, 1), program.run(vec![1]));
        assert_eq!((0, 0, 1, 1), program.run(vec![3]));
        assert_eq!((0, 1, 1, 1), program.run(vec![7]));
        assert_eq!((1, 1, 1, 1), program.run(vec![15]));
    }

    #[test]
    fn part1() {
        let program = Program::parse_program(read_to_string("input.txt").unwrap());

        for serial in (11111111111111i64..=99999999999999i64).rev() {
            let s = format!("{:014}", serial);
            if s.contains("0") {
                continue;
            }

            let nums = s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
            if program.run(nums).3 == 0 {
                assert_eq!(0, serial);
                break;
            }
        }
    }
}
