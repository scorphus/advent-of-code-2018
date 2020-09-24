/// # Day 01 (https://adventofcode.com/2018/day/1)
/// Super easy day. Actually solved part 1 on my terminal, with Fish (❤️🐠🐚):
/// »»»» string join "+ " < data/day01 | math
use crate::input;

use std::collections::HashSet;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part02()
}

struct Day {
    changes: Vec<i32>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            changes: lines
                .iter()
                .map(|l| input::string_to_i32(l.as_ref()))
                .collect(),
        }
    }

    fn part01(&self) -> i32 {
        self.changes.iter().sum()
    }

    fn part02(&self) -> i32 {
        let mut freq = 0;
        let mut it = self.changes.iter().cycle();
        let mut seen_freqs = HashSet::new();
        seen_freqs.insert(freq);
        loop {
            let change = it.next().expect("cyclic iterator exhausted (WTH?)");
            freq += change;
            if seen_freqs.contains(&freq) {
                return freq;
            }
            seen_freqs.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parts {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, changes, expected) = $values;
                    assert_eq!(expected, method(&changes));
                }
            )*
        }
    }

    test_parts! {
        test_part01_01: (part01, vec!["1", "-2", "3", "1"], 3),
        test_part01_02: (part01, vec!["1", "1", "1"], 3),
        test_part01_03: (part01, vec!["1", "1", "-2"], 0),
        test_part01_04: (part01, vec!["-1", "-2", "-3"], -6),
        test_part02_01: (part02, vec!["1", "-1"], 0),
        test_part02_02: (part02, vec!["3", "3", "4", "-2", "-4"], 10),
        test_part02_03: (part02, vec!["-6", "3", "8", "5", "-6"], 5),
        test_part02_04: (part02, vec!["7", "7", "-2", "-7", "-4"], 14),
    }
}
