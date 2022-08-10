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

    fn part01(&mut self) -> Vec<usize> {
        let mut recipes = vec![3, 7];
        let target_len = self.recipes + 10;
        let (mut elf1, mut elf2) = (0, 1);
        while recipes.len() < target_len {
            let new_recipe = recipes[elf1] + recipes[elf2];
            if new_recipe >= 10 {
                recipes.push(new_recipe / 10);
            }
            recipes.push(new_recipe % 10);
            elf1 = (elf1 + 1 + recipes[elf1]) % recipes.len();
            elf2 = (elf2 + 1 + recipes[elf2]) % recipes.len();
        }
        recipes[(target_len - 10)..target_len].to_vec()
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
        test_part01_01: (part01, vec!["9"], "5158916779"),
        test_part01_02: (part01, vec!["5"], "0124515891"),
        test_part01_03: (part01, vec!["18"], "9251071085"),
        test_part01_04: (part01, vec!["2018"], "5941429882"),
        test_part02_01: (part02, vec!["37"], "37"),
    }
}
