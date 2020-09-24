/// # Day 05 (https://adventofcode.com/2018/day/5)
pub fn part01<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part01()
}

struct Day {
    polymer: Vec<char>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            polymer: lines.first().expect("❌").as_ref().chars().collect(),
        }
    }

    fn part01(&self) -> i32 {
        let react = 'a' as i8 - 'A' as i8;
        self.polymer
            .iter()
            .fold(vec![], |mut polymer, &unit| {
                match polymer.last() {
                    Some(&last) if (last as i8 - unit as i8).abs() == react => {
                        polymer.pop();
                    }
                    _ => polymer.push(unit),
                };
                polymer
            })
            .len() as i32
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
                    assert_eq!(expected, method(&records));
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
    }
}
