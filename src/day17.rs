/// Day 17 (https://adventofcode.com/2018/day/17)
extern crate text_io;

use text_io::scan;

use std::collections::HashSet;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    clay: HashSet<Location>,
    min_y: usize,
    max_y: usize,
}

type Location = (usize, usize);

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day {
            min_y: usize::max_value(),
            ..Day::default()
        };
        let mut xy: String;
        let (mut v1, mut v2, mut v3) = (0, 0, 0);
        for line in lines.iter() {
            scan!(line.as_ref().bytes() => "{}={}, {}={}..{}", xy, v1, xy, v2, v3);
            if xy == "x" {
                day.set_min_max(v1, v1);
                for v in v2..=v3 {
                    day.clay.insert((v, v1));
                }
            } else {
                day.set_min_max(v2, v3);
                for v in v2..=v3 {
                    day.clay.insert((v1, v));
                }
            }
        }
        day
    }

    fn set_min_max(&mut self, y0: usize, y1: usize) {
        self.min_y = std::cmp::min(self.min_y, y0);
        self.max_y = std::cmp::max(self.max_y, y1);
    }

    fn part01(&self) -> usize {
        0
    }

    fn part02(&self) -> usize {
        0
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
                    let (method, records, expected) = $values;
                    assert_eq!(method(&records), expected);
                }
            )*
        }
    }

    test_parts! {
        test_part01_01: (part01, vec![
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ], 0),
        test_part02_01: (part02, vec![
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ], 0),
    }
}
