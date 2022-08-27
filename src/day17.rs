/// Day 17 (https://adventofcode.com/2018/day/17)
extern crate text_io;

use text_io::scan;

use std::collections::HashMap;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    ground: HashMap<Location, char>,
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
                    day.ground.insert((v, v1), '#');
                }
            } else {
                day.set_min_max(v2, v3);
                for v in v2..=v3 {
                    day.ground.insert((v1, v), '#');
                }
            }
        }
        day
    }

    fn set_min_max(&mut self, y0: usize, y1: usize) {
        self.min_y = std::cmp::min(self.min_y, y0);
        self.max_y = std::cmp::max(self.max_y, y1);
    }

    fn part01(&mut self) -> usize {
        self.drain(500, 0);
        self.ground
            .iter()
            .filter(|(&(_, y), &c)| self.min_y <= y && y <= self.max_y && (c == '~' || c == '|'))
            .count()
    }

    fn part02(&self) -> usize {
        0
    }

    fn drain(&mut self, x: usize, y: usize) {
        if y > self.max_y || self.is_blocked(x, y) {
            return;
        }
        if self.is_sand(x, y) {
            self.ground.insert((x, y), '|');
            self.drain(x, y + 1);
        }
        if self.is_blocked(x, y + 1) {
            let mut xl = x;
            while self.is_blocked(xl, y + 1) && !self.is_blocked(xl, y) {
                self.ground.insert((xl, y), '|');
                xl -= 1;
            }
            let mut xr = x + 1;
            while self.is_blocked(xr, y + 1) && !self.is_blocked(xr, y) {
                self.ground.insert((xr, y), '|');
                xr += 1;
            }
            if self.is_blocked(xl, y + 1) && self.is_blocked(xr, y + 1) {
                for x in xl + 1..xr {
                    self.ground.insert((x, y), '~');
                }
            } else {
                self.drain(xl, y);
                self.drain(xr, y);
            }
        }
    }

    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.ground
            .get(&(x, y))
            .map_or(false, |c| *c == '#' || *c == '~')
    }

    fn is_sand(&self, x: usize, y: usize) -> bool {
        !self.ground.contains_key(&(x, y))
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
        ], 57),
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
