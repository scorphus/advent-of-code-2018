/// Day 15 (https://adventofcode.com/2018/day/15)
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    open_squares: HashSet<Position>,
    units: HashMap<Position, Unit>,
    order: BinaryHeap<Position>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.y.cmp(&self.y).then_with(|| other.x.cmp(&self.x))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Unit = (char, isize);

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day::default();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.as_ref().chars().enumerate() {
                if c != '#' {
                    day.add_element(x, y, c);
                }
            }
        }
        day
    }

    fn add_element(&mut self, x: usize, y: usize, c: char) {
        let position = Position {
            x: x as isize,
            y: y as isize,
        };
        self.open_squares.insert(position);
        if c != '.' {
            let unit = (c, 200);
            self.units.insert(position, unit);
            self.order.push(position);
        }
    }

    fn part01(&self) -> isize {
        for position in self.order.iter() {
            let (kind, hp) = self.units[position];
            println!("{}({}) at {:?}", kind, hp, position);
        }
        0
    }

    fn part02(&self) -> isize {
        for position in self.order.iter() {
            let (kind, hp) = self.units[position];
            println!("{}({}) at {:?}", kind, hp, position);
        }
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
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ], 0),
        test_part02_01: (part02, vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ], 0),
    }
}
