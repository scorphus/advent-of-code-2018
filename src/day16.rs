/// Day 16 (https://adventofcode.com/2018/day/16)
extern crate text_io;

use text_io::scan;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default, PartialEq)]
struct Day {
    samples: Vec<Sample>,
}

#[derive(Debug, Default, PartialEq)]
struct Sample {
    before: Register,
    instruction: Instruction,
    after: Register,
}

type Register = [isize; 4];

#[derive(Debug, Default, PartialEq)]
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
        }
    }

    fn part01(&self) -> isize {
        0
    }

    fn part02(&self) -> isize {
        0
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
        ], 0),
        test_part02_01: (part02, vec![
            "Before: [3, 2, 1, 1]",
            "9 2 1 2",
            "After:  [3, 2, 2, 1]",
            "",
            "",
        ], 0),
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
}
