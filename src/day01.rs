/// # Day 01 (https://adventofcode.com/2018/day/1)
/// Super easy day. Actually solved part 1 on my terminal, with Fish (❤️🐠🐚):
/// »»»» string join "+ " < data/day01 | math
use crate::input;

use std::collections::HashSet;

struct Day {
    changes: Vec<i32>,
}

impl Day {
    fn read() -> Self {
        let mut changes: Vec<i32> = Vec::new();
        loop {
            let line = input::read_line();
            if line.is_empty() {
                break;
            }
            changes.push(input::string_to_i32(&line));
        }
        Day { changes }
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

pub fn part01() -> i32 {
    Day::read().part01()
}

pub fn part02() -> i32 {
    Day::read().part02()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_day {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, changes, expected) = $values;
                    assert_eq!(expected, method(&Day{ changes }));
                }
            )*
        }
    }

    test_day! {
        test_part01_01: (Day::part01, vec![1, -2, 3, 1], 3),
        test_part01_02: (Day::part01, vec![1, 1, 1], 3),
        test_part01_03: (Day::part01, vec![1, 1, -2], 0),
        test_part01_04: (Day::part01, vec![-1, -2, -3], -6),
        test_part02_01: (Day::part02, vec![1, -1], 0),
        test_part02_02: (Day::part02, vec![3, 3, 4, -2, -4], 10),
        test_part02_03: (Day::part02, vec![-6, 3, 8, 5, -6], 5),
        test_part02_04: (Day::part02, vec![7, 7, -2, -7, -4], 14),
    }
}
