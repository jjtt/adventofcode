use crate::Instruction::*;
use std::fs::read_to_string;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString)]
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
    b_literal: Option<i64>,
}

impl Op {
    fn parse(s: &str) -> Op {
        let p: Vec<&str> = s.split(" ").collect();
        match p[0] {
            "inp" => Op::inp(p[1]),
            _ => Op::op(
                Instruction::from_str(p[0].to_uppercase().as_str()).unwrap(),
                p[1],
                p[2],
            ),
        }
    }

    fn variable_or_literal(variable: &str) -> (usize, Option<i64>) {
        match variable {
            "w" => (0, None),
            "x" => (1, None),
            "y" => (2, None),
            "z" => (3, None),
            _ => (0, variable.parse().ok()),
        }
    }

    fn inp(a: &str) -> Op {
        Op {
            instruction: INP,
            a: Op::variable_or_literal(a).0,
            b: 0,
            b_literal: None,
        }
    }

    fn op(instruction: Instruction, a: &str, b: &str) -> Op {
        let b = Op::variable_or_literal(b);
        Op {
            instruction,
            a: Op::variable_or_literal(a).0,
            b: b.0,
            b_literal: b.1,
        }
    }
}

struct Program {
    ops: Vec<Op>,
    pc: usize,
    variables: [i64; 4],
}

impl Program {
    fn parse_program(s: String) -> Program {
        Program {
            ops: s.lines().map(Op::parse).collect(),
            pc: 0,
            variables: [0; 4],
        }
    }

    fn run(&mut self, input: Vec<i64>) -> (i64, i64, i64, i64) {
        self.reset();
        let mut i = input.iter();
        while !self.end() {
            let input = *i.next().unwrap();
            self.run_one_input(input);
        }
        (
            self.variables[0],
            self.variables[1],
            self.variables[2],
            self.variables[3],
        )
    }

    fn run_one_input(&mut self, input: i64) {
        let mut input_consumed = false;
        while let Some(op) = self.step() {
            match (&op.instruction, input_consumed) {
                (INP, false) => input_consumed = true,
                (INP, true) => {
                    self.unstep();
                    break;
                }
                _ => (),
            }
            let instruction = op.instruction.clone();
            let a = op.a;
            let b_value = op.b_literal.unwrap_or(self.variables[op.b]);
            match instruction {
                INP => self.variables[a] = input,
                ADD => self.variables[a] += b_value,
                MUL => self.variables[a] *= b_value,
                DIV => {
                    assert_ne!(0, b_value);
                    self.variables[a] /= b_value
                }
                MOD => {
                    assert!(self.variables[a] >= 0);
                    assert!(b_value > 0);
                    self.variables[a] %= b_value
                }
                EQL => self.variables[a] = if self.variables[a] == b_value { 1 } else { 0 },
            }
        }
    }

    fn step(&mut self) -> Option<&Op> {
        let op = self.ops.get(self.pc);
        self.pc += 1;
        op
    }

    fn unstep(&mut self) {
        self.pc -= 1;
    }

    fn end(&self) -> bool {
        self.pc >= self.ops.len()
    }

    fn reset(&mut self) -> (usize, [i64; 4]) {
        let state = (self.pc, self.variables);
        self.pc = 0;
        self.variables = [0; 4];
        state
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

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

        assert_eq!(1, program.run(vec![1, 3]).3);
        assert_eq!(1, program.run(vec![2, 6]).3);
        assert_eq!(1, program.run(vec![-2, -6]).3);
        assert_eq!(0, program.run(vec![1, 1]).3);
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

        let mut init_states = HashMap::from([((0, [0i64; 4]), 0); 1]);

        for _ in 0..14 {
            let mut digit_results: HashMap<(usize, [i64; 4]), i64> = HashMap::new();
            for (state, how_to_get_there) in init_states {
                for digit in 1i64..=9i64 {
                    program.pc = state.0;
                    program.variables = state.1;
                    program.run_one_input(digit);
                    let r = program.reset();
                    let current_max = digit_results.get(&r);
                    if current_max.is_none()
                        || *current_max.unwrap() < (how_to_get_there * 10 + digit)
                    {
                        digit_results.insert(r, how_to_get_there * 10 + digit);
                    }
                }
            }
            dbg!(digit_results.len());
            //dbg!(&digit_results);
            init_states = digit_results;
        }

        let valid: Vec<(&(usize, [i64; 4]), &i64)> =
            init_states.iter().filter(|s| s.0 .1[3] == 0).collect();

        assert_eq!(7, valid.len());
        dbg!(&valid);

        let max_serial = valid.iter().map(|(_, serial)| *serial).max().unwrap();

        assert_eq!(98998519596997, *max_serial);
    }
}
