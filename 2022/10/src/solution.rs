use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum InstructionType {
    Noop,
    Addx(i64),
}

#[derive(Debug)]
struct Instruction {
    instr_type: InstructionType,
    cycles: usize,
}

impl Instruction {
    fn noop() -> Instruction {
        Instruction {
            instr_type: InstructionType::Noop,
            cycles: 1,
        }
    }
    fn addx(arg: i64) -> Instruction {
        Instruction {
            instr_type: InstructionType::Addx(arg),
            cycles: 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::noop())
        } else if let Some(arg) = s.strip_prefix("addx ") {
            let arg = arg.parse().unwrap();
            Ok(Instruction::addx(arg))
        } else {
            Err(format!("Unsupported instruction: {s}"))
        }
    }
}

#[derive(Debug)]
struct Cpu {
    register: i64,
    cycle: usize,
    instr_cycle: usize,
    instr: Instruction,
    crt: [bool; 240],
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s: String = self.crt.map(|p| if p { '#' } else { '.' }).iter().collect();
        s.insert(200, '\n');
        s.insert(160, '\n');
        s.insert(120, '\n');
        s.insert(80, '\n');
        s.insert(40, '\n');
        write!(f, "{s}")
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            register: 1,
            cycle: 0,
            instr_cycle: 1,
            instr: Instruction::noop(),
            crt: [false; 240],
        }
    }

    fn tick(&mut self) -> (bool, i64) {
        self.cycle += 1;
        let pixel = (self.cycle - 1) % 240;
        self.crt[pixel] =
            pixel as i64 % 40 >= self.register - 1 && self.register + 1 >= pixel as i64 % 40;
        self.instr_cycle += 1;
        let signal_strength = self.signal_strength();
        match self.instr.instr_type {
            InstructionType::Noop => {}
            InstructionType::Addx(arg) => {
                if self.instr_cycle >= self.instr.cycles {
                    self.register += arg;
                }
            }
        }
        (self.instr_cycle < self.instr.cycles, signal_strength)
    }

    fn set_instr(&mut self, instr: Instruction) {
        assert!(self.instr_cycle >= self.instr.cycles);
        self.instr = instr;
        self.instr_cycle = 0;
    }

    fn signal_strength(&self) -> i64 {
        if (self.cycle as i64 - 20) % 40 == 0 {
            self.cycle as i64 * self.register
        } else {
            0
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let program: Vec<Instruction> = read_to_string(input)
        .unwrap()
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    let mut cpu = Cpu::new();
    let mut signal_strength = 0;
    for instr in program {
        cpu.set_instr(instr);
        loop {
            let (more_ticks, current_signal_strength) = cpu.tick();
            signal_strength += current_signal_strength;
            if !more_ticks {
                break;
            }
        }
    }

    signal_strength
}

pub fn part2(input: &str) -> String {
    let program: Vec<Instruction> = read_to_string(input)
        .unwrap()
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    let mut cpu = Cpu::new();
    for instr in program {
        cpu.set_instr(instr);
        loop {
            let (more_ticks, _) = cpu.tick();
            if !more_ticks {
                break;
            }
        }
    }

    format!("{cpu}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let instr: Instruction = "noop".parse().unwrap();
        assert_eq!(InstructionType::Noop, instr.instr_type);
        assert_eq!(1, instr.cycles);

        let instr: Instruction = "addx -42".parse().unwrap();
        assert_eq!(InstructionType::Addx(-42), instr.instr_type);
        assert_eq!(2, instr.cycles);
    }

    #[test]
    fn ticking() {
        let mut cpu = Cpu::new();

        assert_eq!(0, cpu.cycle);
        assert_eq!(1, cpu.register);

        // Give the CPU something to work on
        cpu.set_instr(Instruction::noop());
        assert!(!cpu.tick().0);
        assert_eq!(1, cpu.cycle);
        assert_eq!(1, cpu.register);

        cpu.set_instr(Instruction::addx(3));
        assert!(cpu.tick().0);
        assert_eq!(2, cpu.cycle);
        assert_eq!(1, cpu.register);
        assert!(!cpu.tick().0);
        assert_eq!(3, cpu.cycle);
        assert_eq!(4, cpu.register);

        cpu.set_instr(Instruction::addx(-5));
        assert!(cpu.tick().0);
        assert_eq!(4, cpu.cycle);
        assert_eq!(4, cpu.register);
        assert!(!cpu.tick().0);
        assert_eq!(5, cpu.cycle);
        assert_eq!(-1, cpu.register);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(13140, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(expected, part2("sample.txt"));
    }
}
