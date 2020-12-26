/// Day 5 (https://adventofcode.com/2018/day/5)
const REACT: i8 = 'a' as i8 - 'A' as i8;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part02()
}

struct Day {
    polymer: Vec<i8>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            polymer: lines
                .first()
                .expect("❌")
                .as_ref()
                .chars()
                .map(|c| c as i8)
                .collect(),
        }
    }

    fn part01(&self) -> i32 {
        self.polymer
            .iter()
            .fold(vec![], |mut polymer: Vec<i8>, &unit| {
                match polymer.last() {
                    Some(&last) if (last - unit).abs() == REACT => {
                        polymer.pop();
                    }
                    _ => polymer.push(unit),
                };
                polymer
            })
            .len() as i32
    }

    fn part02(&self) -> i32 {
        ('a' as i8..'z' as i8 + 1)
            .map(|skip| {
                println!("{}", skip as u8 as char);
                self.polymer
                    .iter()
                    .fold(vec![], |mut polymer: Vec<i8>, &unit| {
                        if unit == skip || (unit - skip).abs() == REACT {
                            return polymer;
                        }
                        match polymer.last() {
                            Some(&last) if (last - unit).abs() == REACT => {
                                polymer.pop();
                            }
                            _ => polymer.push(unit),
                        };
                        polymer
                    })
                    .len() as i32
            })
            .min()
            .expect("")
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
        test_part01_01: (part01, vec!["aA"], 0),
        test_part01_02: (part01, vec!["abBA"], 0),
        test_part01_03: (part01, vec!["abAB"], 4),
        test_part01_04: (part01, vec!["aabAAB"], 6),
        test_part01_05: (part01, vec!["dabCBAcaDA"], 10),
        test_part01_06: (part01, vec!["dabCBAcCcaDA"], 10),
        test_part01_07: (part01, vec!["dabAaCBAcCcaDA"], 10),
        test_part01_08: (part01, vec!["dabAcCaCBAcCcaDA"], 10),
        test_part02_01: (part02, vec!["aA"], 0),
        test_part02_02: (part02, vec!["abBA"], 0),
        test_part02_03: (part02, vec!["abAB"], 0),
        test_part02_04: (part02, vec!["aabAAB"], 0),
        test_part02_05: (part02, vec!["dabCBAcaDA"], 4),
        test_part02_06: (part02, vec!["dabCBAcCcaDA"], 4),
        test_part02_07: (part02, vec!["dabAaCBAcCcaDA"], 4),
        test_part02_08: (part02, vec!["dabAcCaCBAcCcaDA"], 4),
    }
}
