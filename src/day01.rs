/// # Day 01 (https://adventofcode.com/2018/day/1)
/// Super easy day. Actually solved it on my terminal, with Fish:
/// »»»» string join "+ " < data/day01 | math # ❤️🐠🐚

use crate::input;

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
}

pub fn part01() -> i32 {
    let data = Day::read();
    data.part01()
}
