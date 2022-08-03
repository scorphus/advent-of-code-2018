/// Day 11 (https://adventofcode.com/2018/day/11)
extern crate text_io;

use std::collections::HashMap;

const GRID_SIZE: isize = 300;
const SQUARE_SIZE: isize = 3;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part01();
    format!("{},{}", x, y)
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part02();
    format!("{},{}", x, y)
}

type Cell = (isize, isize);

#[derive(Debug, Default)]
struct Day {
    grid_serial_number: isize,
    power_levels: HashMap<Cell, isize>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day {
            grid_serial_number: lines.first().expect("❌").as_ref().parse().expect("❌"),
            ..Default::default()
        };
        day.fill_power_levels();
        day
    }

    fn fill_power_levels(&mut self) {
        for x in 1..=GRID_SIZE {
            for y in 1..=GRID_SIZE {
                self.power_levels.insert((x, y), self.power_level((x, y)));
            }
        }
    }

    fn power_level(&self, (x, y): Cell) -> isize {
        let rack_id = x + 10;
        let power_level = rack_id * y + self.grid_serial_number;
        power_level * rack_id / 100 % 10 - 5
    }

    fn part01(&self) -> Cell {
        let mut max_power_level = isize::MIN;
        let mut max_cell = (0, 0);
        for x in 1..=GRID_SIZE - SQUARE_SIZE {
            for y in 1..=GRID_SIZE - SQUARE_SIZE {
                let mut power_level = 0;
                for dx in 0..SQUARE_SIZE {
                    for dy in 0..SQUARE_SIZE {
                        power_level += self.power_levels[&(x + dx, y + dy)];
                    }
                }
                if power_level > max_power_level {
                    max_power_level = power_level;
                    max_cell = (x, y);
                }
            }
        }
        max_cell
    }

    fn part02(&self) -> Cell {
        (self.grid_serial_number, self.power_level((179, 359)))
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
        test_part01_01: (part01, vec!["18"], "33,45"),
        test_part01_02: (part01, vec!["42"], "21,61"),
    }

    macro_rules! test_power_level {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (cell, grid_serial_number, expected) = $values;
                    let day = Day { grid_serial_number, ..Default::default() };
                    assert_eq!(day.power_level(cell), expected);
                }
            )*
        }
    }

    test_power_level! {
        test_power_level_01: ((3, 5), 8, 4),
        test_power_level_02: ((122,79), 57, -5),
        test_power_level_03: ((217,196), 39, 0),
        test_power_level_04: ((101,153), 71, 4),
    }

    macro_rules! test_fill_power_levels {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (cell, grid_serial_number, expected) = $values;
                    let mut day = Day { grid_serial_number, ..Default::default() };
                    day.fill_power_levels();
                    assert_eq!(day.power_levels.len(), (GRID_SIZE * GRID_SIZE) as usize);
                    assert_eq!(day.power_levels[&cell], expected);
                }
            )*
        }
    }

    test_fill_power_levels! {
        test_fill_power_levels_01: ((3, 5), 8, 4),
        test_fill_power_levels_02: ((122,79), 57, -5),
        test_fill_power_levels_03: ((217,196), 39, 0),
        test_fill_power_levels_04: ((101,153), 71, 4),
    }
}
