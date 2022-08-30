/// Day 19 (https://adventofcode.com/2018/day/19)
extern crate text_io;

use text_io::scan;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    instructions: Vec<Instruction>,
    register: Register,
    ip: usize,
}

type Register = [isize; 6];

#[derive(Debug, Default)]
struct Instruction {
    opcode: usize,
    a: isize,
    b: isize,
    c: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut iter = lines.iter();
        let mut ip;
        scan!(iter.next().expect("❌").as_ref().bytes() => "#ip {}", ip);
        Day {
            instructions: iter.map(|l| parse_instruction(l.as_ref())).collect(),
            ip,
            ..Day::default()
        }
    }

    fn part01(&mut self) -> isize {
        let mut ip = self.register[self.ip];
        while ip < self.instructions.len() as isize {
            self.register[self.ip] = ip;
            let instruction = &self.instructions[ip as usize];
            self.register = run_instruction(instruction, self.register);
            ip = self.register[self.ip] + 1;
        }
        self.register[0]
    }

    fn part02(&mut self) -> isize {
        self.register[0] = 1;
        let mut ip = self.register[self.ip];
        while ip < self.instructions.len() as isize && self.register[0] != 0 {
            self.register[self.ip] = ip;
            let instruction = &self.instructions[ip as usize];
            self.register = run_instruction(instruction, self.register);
            ip = self.register[self.ip] + 1;
        }
        let register_max = self.register.iter().max().expect("❌");
        let factors = factorize(*register_max);
        factors.iter().sum()
    }
}

fn parse_instruction(string: &str) -> Instruction {
    let mut i = Instruction::default();
    let mut opcode: String;
    scan!(string.bytes() => "{} {} {} {}", opcode, i.a, i.b, i.c);
    i.opcode = match opcode.as_str() {
        "addr" => 0,
        "addi" => 1,
        "mulr" => 2,
        "muli" => 3,
        "banr" => 4,
        "bani" => 5,
        "borr" => 6,
        "bori" => 7,
        "setr" => 8,
        "seti" => 9,
        "gtir" => 10,
        "gtri" => 11,
        "gtrr" => 12,
        "eqir" => 13,
        "eqri" => 14,
        "eqrr" => 15,
        _ => unreachable!(),
    };
    i
}

fn factorize(n: isize) -> Vec<isize> {
    let mut factors = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i);
            }
        }
        i += 1;
    }
    factors
}

#[rustfmt::skip]
fn run_instruction(instruction: &Instruction, mut register: Register) -> Register {
    let (opcode, a, b, c) = (&instruction.opcode, instruction.a, instruction.b, instruction.c);
    register[c] = match (opcode, a as usize, b as usize) {
        ( 0, ai, bi) => register[ai] + register[bi],                      // addr
        ( 1, ai,  _) => register[ai] + b,                                 // addi
        ( 2, ai, bi) => register[ai] * register[bi],                      // mulr
        ( 3, ai,  _) => register[ai] * b,                                 // muli
        ( 4, ai, bi) => register[ai] & register[bi],                      // banr
        ( 5, ai,  _) => register[ai] & b,                                 // bani
        ( 6, ai, bi) => register[ai] | register[bi],                      // borr
        ( 7, ai,  _) => register[ai] | b,                                 // bori
        ( 8, ai,  _) => register[ai],                                     // setr
        ( 9,  _,  _) => a,                                                // seti
        (10,  _, bi) => if a > register[bi]             { 1 } else { 0 }, // gtir
        (11, ai,  _) => if register[ai] > b             { 1 } else { 0 }, // gtri
        (12, ai, bi) => if register[ai] > register[bi]  { 1 } else { 0 }, // gtrr
        (13,  _, bi) => if a == register[bi]            { 1 } else { 0 }, // eqir
        (14, ai,  _) => if register[ai] == b            { 1 } else { 0 }, // eqri
        (15, ai, bi) => if register[ai] == register[bi] { 1 } else { 0 }, // eqrr
        _ => unreachable!(),
    };
    register
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parts {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, records, expected) = $values;
                    assert_eq!(method(&records), expected);
                }
            )*
        }
    }

    test_parts! {
        test_part01_01: (part01, vec![
            "#ip 0",
            "seti 5 0 1",
            "seti 6 0 2",
            "addi 0 1 0",
            "addr 1 2 3",
            "setr 1 0 0",
            "seti 8 0 4",
            "seti 9 0 5",
        ], 6),
        test_part01_02: (part01, crate::input::read_lines_from_input("data/day19"), 948),
        test_part02_01: (part02, crate::input::read_lines_from_input("data/day19"), 10695960),
    }

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(1), vec![1]);
        assert_eq!(factorize(2), vec![1, 2]);
        assert_eq!(factorize(3), vec![1, 3]);
        assert_eq!(factorize(4), vec![1, 4, 2]);
        assert_eq!(factorize(5), vec![1, 5]);
        assert_eq!(factorize(6), vec![1, 6, 2, 3]);
        assert_eq!(factorize(7), vec![1, 7]);
        assert_eq!(factorize(8), vec![1, 8, 2, 4]);
        assert_eq!(factorize(9), vec![1, 9, 3]);
    }
}
