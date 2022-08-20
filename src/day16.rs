/// Day 16 (https://adventofcode.com/2018/day/16)
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
    things: Vec<Something>,
}

#[derive(Debug, PartialEq)]
struct Something {
    a: String,
    b: i32,
    c: f32,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            // lines: lines.iter().map(|l| l.as_ref().to_string()).collect(),
            things: lines.iter().map(|l| parse_something(l.as_ref())).collect(),
        }
    }

    fn part01(&self) -> isize {
        0
    }

    fn part02(&self) -> isize {
        0
    }
}

fn parse_something(string: &str) -> Something {
    let mut c = Something {
        a: String::from(""),
        b: 0,
        c: 0.0,
    };
    scan!(string.bytes() => "{} {} {}", c.a, c.b, c.c);
    c
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
        test_part01_01: (part01, vec!["foo 359 0.4986"], 359),
        test_part02_01: (part02, vec!["bar 179 2.0056"], 179),
    }
}
