/// Day 16 (https://adventofcode.com/2018/day/16)
extern crate text_io;

use std::collections::HashMap;
use text_io::scan;

const TOTAL_INSTRUCTIONS: usize = 16;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default, PartialEq)]
struct Day {
    samples: Vec<Sample>,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Default, PartialEq)]
struct Sample {
    before: Register,
    instruction: Instruction,
    after: Register,
}

type Register = [isize; 4];

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Instruction {
    opcode: usize,
    a: isize,
    b: isize,
    c: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let split_index = find_split_index(lines);
        Day {
            samples: lines[..split_index]
                .chunks(4)
                .map(|l| parse_sample(l))
                .collect(),
            instructions: lines[split_index + 3..]
                .iter()
                .map(|l| parse_instruction(l.as_ref()))
                .collect(),
        }
    }

    fn part01(&self) -> isize {
        let mut matching_samples = 0;
        for sample in &self.samples {
            let mut total_matching_opcodes = 0;
            for i in 0..TOTAL_INSTRUCTIONS {
                if run_instruction(i, sample.before, sample.instruction) == sample.after {
                    total_matching_opcodes += 1;
                    if total_matching_opcodes == 3 {
                        matching_samples += 1;
                        break;
                    }
                }
            }
        }
        matching_samples
    }

    fn part02(&self) -> isize {
        let mut sample_to_opcode = HashMap::new();
        let mut opcode_to_sample = HashMap::new();
        for sample in &self.samples {
            if sample_to_opcode.contains_key(&sample.instruction.opcode) {
                continue;
            }
            let mut matching_opcode = 0;
            let mut total_matching_opcodes = 0;
            for i in 0..TOTAL_INSTRUCTIONS {
                if opcode_to_sample.contains_key(&i) {
                    continue;
                }
                if run_instruction(i, sample.before, sample.instruction) == sample.after {
                    matching_opcode = i;
                    total_matching_opcodes += 1;
                }
            }
            if total_matching_opcodes == 1 {
                sample_to_opcode.insert(sample.instruction.opcode, matching_opcode);
                opcode_to_sample.insert(matching_opcode, sample.instruction.opcode);
            }
        }
        let mut register = [0; 4];
        for instruction in &self.instructions {
            let opcode = sample_to_opcode[&instruction.opcode];
            register = run_instruction(opcode, register, *instruction);
        }
        register[0]
    }
}

fn find_split_index<T: AsRef<str>>(lines: &[T]) -> usize {
    let mut iter = lines.iter();
    let mut last_line = iter.next().unwrap().as_ref();
    for (i, line) in iter.enumerate() {
        if line.as_ref() == last_line {
            return i;
        }
        last_line = line.as_ref();
    }
    0
}

fn parse_sample<T: AsRef<str>>(slice: &[T]) -> Sample {
    let mut s = Sample::default();
    let mut it = slice.iter();
    scan!(it.next().unwrap().as_ref().bytes() => "Before: [{}, {}, {}, {}]", s.before[0], s.before[1], s.before[2], s.before[3]);
    scan!(it.next().unwrap().as_ref().bytes() => "{} {} {} {}", s.instruction.opcode, s.instruction.a, s.instruction.b, s.instruction.c);
    scan!(it.next().unwrap().as_ref().bytes() => "After:  [{}, {}, {}, {}]", s.after[0], s.after[1], s.after[2], s.after[3]);
    s
}

fn parse_instruction(line: &str) -> Instruction {
    let mut instruction = Instruction::default();
    scan!(line.bytes() => "{} {} {} {}", instruction.opcode, instruction.a, instruction.b, instruction.c);
    instruction
}

#[rustfmt::skip]
fn run_instruction(i: usize, mut register: Register, instr: Instruction) -> Register {
    let (a, b, c) = (instr.a, instr.b, instr.c);
    register[c] = match (i, a as usize, b as usize) {
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
        ( _,  _,  _) => 0,
    };
    register
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from() {
        assert_eq!(
            Day::read_from(&[
                "Before: [3, 2, 1, 1]",
                "9 2 1 2",
                "After:  [3, 2, 2, 1]",
                "",
                "",
                "",
                "9 2 1 2",
                "9 3 2 1",
            ]),
            Day {
                samples: vec![Sample {
                    before: [3, 2, 1, 1],
                    instruction: Instruction {
                        opcode: 9,
                        a: 2,
                        b: 1,
                        c: 2,
                    },
                    after: [3, 2, 2, 1],
                }],
                instructions: vec![
                    Instruction {
                        opcode: 9,
                        a: 2,
                        b: 1,
                        c: 2,
                    },
                    Instruction {
                        opcode: 9,
                        a: 3,
                        b: 2,
                        c: 1,
                    },
                ],
            },
        );
    }

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
            "Before: [3, 2, 1, 1]",
            "9 2 1 2",
            "After:  [3, 2, 2, 1]",
            "",
            "",
            "",
            "9 2 1 2",
        ], 1),
        test_part01_02: (part01, crate::input::read_lines_from_input("data/day16"), 570),
        test_part02_01: (part02, crate::input::read_lines_from_input("data/day16"), 503),
    }

    #[test]
    fn test_find_split_index() {
        assert_eq!(
            find_split_index(&[
                "Before: [3, 2, 1, 1]",
                "9 2 1 2",
                "After:  [3, 2, 2, 1]",
                "",
                "",
            ]),
            3,
        );
    }

    #[test]
    fn test_parse_sample() {
        assert_eq!(
            parse_sample(&["Before: [3, 2, 1, 1]", "9 2 1 2", "After:  [3, 2, 2, 1]"]),
            Sample {
                before: [3, 2, 1, 1],
                instruction: Instruction {
                    opcode: 9,
                    a: 2,
                    b: 1,
                    c: 2,
                },
                after: [3, 2, 2, 1],
            },
        );
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("9 2 1 2"),
            Instruction {
                opcode: 9,
                a: 2,
                b: 1,
                c: 2,
            },
        );
    }

    macro_rules! test_run_instruction {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (i, expected) = $values;
                    assert_eq!(
                        run_instruction(i, [3, 2, 1, 1], Instruction {
                            opcode: 9,
                            a: 2,
                            b: 1,
                            c: 2,
                        }),
                        expected,
                    );
                }
            )*
        }
    }

    test_run_instruction! {
        test_run_instruction_00: (0, [3, 2, 3, 1]),
        test_run_instruction_01: (1, [3, 2, 2, 1]),
        test_run_instruction_02: (2, [3, 2, 2, 1]),
        test_run_instruction_03: (3, [3, 2, 1, 1]),
        test_run_instruction_04: (4, [3, 2, 0, 1]),
        test_run_instruction_05: (5, [3, 2, 1, 1]),
        test_run_instruction_06: (6, [3, 2, 3, 1]),
        test_run_instruction_07: (7, [3, 2, 1, 1]),
        test_run_instruction_08: (8, [3, 2, 1, 1]),
        test_run_instruction_09: (9, [3, 2, 2, 1]),
        test_run_instruction_10: (10, [3, 2, 0, 1]),
        test_run_instruction_11: (11, [3, 2, 0, 1]),
        test_run_instruction_12: (12, [3, 2, 0, 1]),
        test_run_instruction_13: (13, [3, 2, 1, 1]),
        test_run_instruction_14: (14, [3, 2, 1, 1]),
        test_run_instruction_15: (15, [3, 2, 0, 1]),
    }
}
