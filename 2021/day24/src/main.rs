use itertools::Itertools;
use std::env::var;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
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
    a: usize,
    b: usize,
    b_literal: Option<i32>,
}

impl Op {
    fn parse(s: &str) -> Op {
        let p: Vec<&str> = s.split(" ").collect();
        match p[0] {
            "inp" => Op::inp(p[1]),
            "mul" => Op::mul(p[1], p[2]),
            _ => todo!("{}", p[0]),
        }
    }

    fn variable_or_literal(variable: &str) -> (usize, Option<i32>) {
        match variable {
            "w" => (0, None),
            "x" => (1, None),
            "y" => (2, None),
            "z" => (3, None),
            _ => (0, variable.parse::<i32>().ok()),
        }
    }

    fn inp(a: &str) -> Op {
        Op {
            instruction: Instruction::INP,
            a: Op::variable_or_literal(a).0,
            b: 0,
            b_literal: None,
        }
    }

    fn mul(a: &str, b: &str) -> Op {
        let b = Op::variable_or_literal(b);
        Op {
            instruction: Instruction::MUL,
            a: Op::variable_or_literal(a).0,
            b: b.0,
            b_literal: b.1,
        }
    }
}

struct Program {
    ops: Vec<Op>,
    pc: usize,
    variables: [i32; 4],
}

impl Program {
    fn parse_program(s: String) -> Program {
        Program {
            ops: s.lines().map(Op::parse).collect(),
            pc: 0,
            variables: [0; 4],
        }
    }

    fn run(&mut self, input: Vec<i32>) -> (i32, i32, i32, i32) {
        self.reset();
        let mut i = input.iter();
        while let Some(op) = self.step() {
            let instruction = op.instruction.clone();
            let a = op.a;
            let b_value = op.b_literal.unwrap_or(self.variables[op.b]);
            match instruction {
                Instruction::INP => self.variables[a] = *i.next().unwrap(),
                Instruction::MUL => self.variables[a] *= b_value,
                _ => panic!("Running {:?} not implemented", instruction),
            }
        }
        (
            self.variables[0],
            self.variables[1],
            self.variables[2],
            self.variables[3],
        )
    }

    fn step(&mut self) -> Option<&Op> {
        let op = self.ops.get(self.pc);
        self.pc += 1;
        op
    }
    fn reset(&mut self) {
        self.pc = 0;
        self.variables = [0; 4];
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
        let mut program = Program::parse_program(read_to_string("sample1.txt").unwrap());

        assert_eq!(-1, program.run(vec![1]).1);
        assert_eq!(1, program.run(vec![-1]).1);
        assert_eq!(-42, program.run(vec![42]).1);
    }

    #[test]
    fn sample2() {
        let mut program = Program::parse_program(read_to_string("sample2.txt").unwrap());

        assert_eq!(1, program.run(vec![1, 3]).1);
        assert_eq!(1, program.run(vec![2, 6]).1);
        assert_eq!(1, program.run(vec![-2, -6]).1);
        assert_eq!(0, program.run(vec![1, 1]).1);
    }

    #[test]
    fn sample3() {
        let mut program = Program::parse_program(read_to_string("sample3.txt").unwrap());

        assert_eq!((0, 0, 0, 0), program.run(vec![0]));
        assert_eq!((0, 0, 0, 1), program.run(vec![1]));
        assert_eq!((0, 0, 1, 1), program.run(vec![3]));
        assert_eq!((0, 1, 1, 1), program.run(vec![7]));
        assert_eq!((1, 1, 1, 1), program.run(vec![15]));
    }

    #[test]
    fn part1() {
        let mut program = Program::parse_program(read_to_string("input.txt").unwrap());

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
