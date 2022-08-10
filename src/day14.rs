/// Day 14 (https://adventofcode.com/2018/day/14)
extern crate text_io;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    Day::read_from(lines)
        .part01()
        .iter()
        .map(|&id| id.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> String {
    Day::read_from(lines)
        .part02()
        .iter()
        .map(|&id| id.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug, Default)]
struct Day {
    recipes: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            recipes: lines.first().expect("❌").as_ref().parse().expect("❌"),
        }
    }

    fn part01(&self) -> Vec<usize> {
        println!("{:?}", self.recipes);
        vec![3, 7]
    }

    fn part02(&self) -> Vec<usize> {
        println!("{:?}", self.recipes);
        vec![3, 7]
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
        test_part01_01: (part01, vec!["37"], "37"),
        test_part02_01: (part02, vec!["37"], "37"),
    }
}
