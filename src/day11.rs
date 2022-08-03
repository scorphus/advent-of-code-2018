/// Day 11 (https://adventofcode.com/2018/day/11)
extern crate text_io;

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
    grid_serial_number: isize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            grid_serial_number: lines.first().expect("❌").as_ref().parse().expect("❌"),
        }
    }

    fn part01(&self) -> (isize, isize) {
        (self.grid_serial_number, self.grid_serial_number)
    }

    fn part02(&self) -> (isize, isize) {
        (self.grid_serial_number, self.grid_serial_number)
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
        test_part01_01: (part01, vec!["18"], "18,18"),
        test_part02_01: (part02, vec!["42"], "42,42"),
    }
}
