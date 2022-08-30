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

    fn part01(&self) -> isize {
        0
    }

    fn part02(&self) -> isize {
        0
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
        ], 0),
        test_part02_01: (part02, vec![
            "#ip 0",
            "seti 5 0 1",
            "seti 6 0 2",
            "addi 0 1 0",
            "addr 1 2 3",
            "setr 1 0 0",
            "seti 8 0 4",
            "seti 9 0 5",
        ], 0),
    }
}
