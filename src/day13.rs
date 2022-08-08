/// Day 13 (https://adventofcode.com/2018/day/13)
extern crate text_io;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part01();
    format!("{},{}", x, y)
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part02();
    format!("{},{}", x, y)
}

#[derive(Debug, Default)]
struct Day {
    tracks: HashMap<Location, char>,
    carts: BinaryHeap<Cart>,
}

type Location = (usize, usize);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Cart {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        other.y.cmp(&self.y).then_with(|| other.x.cmp(&self.x))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day::default();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.as_ref().chars().enumerate() {
                if c != ' ' {
                    day.add_element(x, y, c);
                }
            }
        }
        day
    }

    fn add_element(&mut self, x: usize, y: usize, c: char) {
        let (vx, vy, new_c) = match c {
            '^' => (0, -1, '|'),
            'v' => (0, 1, '|'),
            '<' => (-1, 0, '-'),
            '>' => (1, 0, '-'),
            _ => (0, 0, c),
        };
        self.tracks.insert((x, y), new_c);
        if new_c != c {
            self.carts.push(Cart { x, y, vx, vy });
        }
    }

    fn part01(&mut self) -> Location {
        for cart in &self.carts {
            println!("{},{},{},{}", cart.x, cart.y, cart.vx, cart.vy);
        }
        (0, 0)
    }

    fn part02(&mut self) -> Location {
        for cart in &self.carts {
            println!("{},{},{},{}", cart.x, cart.y, cart.vx, cart.vy);
        }
        (0, 0)
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
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
        ], "0,0"),
        test_part02_01: (part02, vec![
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
        ], "0,0"),
    }
}
