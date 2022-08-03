/// Day 11 (https://adventofcode.com/2018/day/11)
extern crate text_io;

const GRID_SIZE: usize = 300;
const SQUARE_SIZE: usize = 3;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part01();
    format!("{},{}", x, y)
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part02();
    format!("{},{}", x, y)
}

type Cell = (usize, usize);

#[derive(Debug, Default)]
struct Day {
    grid_serial_number: isize,
    prefixed_power_levels: Vec<Vec<isize>>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day {
            grid_serial_number: lines.first().expect("❌").as_ref().parse().expect("❌"),
            prefixed_power_levels: vec![vec![0; GRID_SIZE + 2]; GRID_SIZE + 2],
        };
        day.fill_prefixed_power_levels();
        day
    }

    fn fill_prefixed_power_levels(&mut self) {
        for x in (1..=GRID_SIZE).rev() {
            for y in (1..=GRID_SIZE).rev() {
                self.prefixed_power_levels[x][y] = self.power_level((x, y))
                    + self.prefixed_power_levels[x][y + 1]
                    + self.prefixed_power_levels[x + 1][y]
                    - self.prefixed_power_levels[x + 1][y + 1];
            }
        }
    }

    fn power_level(&self, (x, y): Cell) -> isize {
        let rack_id = x as isize + 10;
        let power_level = rack_id * y as isize + self.grid_serial_number;
        power_level * rack_id / 100 % 10 - 5
    }

    fn part01(&self) -> Cell {
        let mut max_power_level = isize::MIN;
        let mut max_cell = (0, 0);
        for x in SQUARE_SIZE..=GRID_SIZE - SQUARE_SIZE {
            for y in SQUARE_SIZE..=GRID_SIZE - SQUARE_SIZE {
                let power_level = self.prefixed_power_levels[x][y]
                    - self.prefixed_power_levels[x][y + SQUARE_SIZE]
                    - self.prefixed_power_levels[x + SQUARE_SIZE][y]
                    + self.prefixed_power_levels[x + SQUARE_SIZE][y + SQUARE_SIZE];
                if power_level > max_power_level {
                    max_power_level = power_level;
                    max_cell = (x, y);
                }
            }
        }
        max_cell
    }

    fn part02(&self) -> Cell {
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

    macro_rules! test_fill_prefixed_power_levels {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((x, y), grid_serial_number, expected) = $values;
                    let mut day = Day {
                        grid_serial_number,
                        prefixed_power_levels: vec![vec![0; GRID_SIZE + 2]; GRID_SIZE + 2],
                    };
                    day.fill_prefixed_power_levels();
                    let total_power = day.prefixed_power_levels[x][y]
                        - day.prefixed_power_levels[x][y + SQUARE_SIZE]
                        - day.prefixed_power_levels[x + SQUARE_SIZE][y]
                        + day.prefixed_power_levels[x + SQUARE_SIZE][y + SQUARE_SIZE];
                    assert_eq!(total_power, expected);
                }
            )*
        }
    }

    test_fill_prefixed_power_levels! {
        test_fill_prefixed_power_levels_01: ((33,45), 18, 29),
        test_fill_prefixed_power_levels_02: ((21,61), 42, 30),
    }
}
