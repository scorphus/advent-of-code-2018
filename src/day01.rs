/// # Day 01 (https://adventofcode.com/2018/day/1)
/// Super easy day. Actually solved part 1 on my terminal, with Fish (❤️🐠🐚):
/// »»»» string join "+ " < data/day01 | math
use crate::input;

use std::collections::HashSet;

struct Day {
    frequencies: Vec<i32>,
}

impl Day {
    fn read() -> Self {
        let mut frequencies: Vec<i32> = Vec::new();
        loop {
            let line = input::read_line();
            if line.is_empty() {
                break;
            }
            frequencies.push(input::string_to_i32(&line));
        }
        Day { frequencies }
    }

    fn part01(&self) -> i32 {
        self.frequencies.iter().sum()
    }

    fn part02(&self) -> i32 {
        let mut freq = 0;
        let mut it = self.frequencies.iter().cycle();
        let mut seen_freqs = HashSet::new();
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
