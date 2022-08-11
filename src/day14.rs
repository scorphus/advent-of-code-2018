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

pub fn part02<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    recipes: Vec<usize>,
    target_len: usize,
    matched: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let first_line = lines.first().expect("❌").as_ref();
        Day {
            recipes: first_line
                .chars()
                .map(|c| c.to_digit(10).expect("❌") as usize)
                .collect(),
            target_len: first_line.parse::<usize>().expect("❌") + 10,
            matched: 0,
        }
    }

    fn part01(&mut self) -> Vec<usize> {
        self.create_new_recipes_until(|s, r, _| r.len() >= s.target_len)[self.target_len - 10..]
            .to_vec()
    }

    fn part02(&mut self) -> usize {
        self.create_new_recipes_until(Day::has_match).len() - self.recipes.len()
    }

    fn create_new_recipes_until<F>(&mut self, predicate: F) -> Vec<usize>
    where
        F: Fn(&mut Day, &Vec<usize>, usize) -> bool,
    {
        let mut recipes = vec![3, 7];
        let (mut elf1, mut elf2) = (0, 1);
        loop {
            let new_recipe = recipes[elf1] + recipes[elf2];
            if new_recipe >= 10 {
                recipes.push(new_recipe / 10);
                if predicate(self, &recipes, new_recipe / 10) {
                    break;
                }
            }
            recipes.push(new_recipe % 10);
            if predicate(self, &recipes, new_recipe % 10) {
                break;
            }
            elf1 = (elf1 + 1 + recipes[elf1]) % recipes.len();
            elf2 = (elf2 + 1 + recipes[elf2]) % recipes.len();
        }
        recipes
    }

    fn has_match(&mut self, _: &Vec<usize>, recipe: usize) -> bool {
        if self.matched < self.recipes.len() && self.recipes[self.matched] == recipe {
            self.matched += 1;
            return self.matched == self.recipes.len();
        }
        self.matched = 0;
        if self.recipes[0] == recipe {
            self.matched = 1;
        }
        false
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
        test_part02_01: (part02, vec!["51589"], 9),
        test_part02_02: (part02, vec!["01245"], 5),
        test_part02_03: (part02, vec!["92510"], 18),
        test_part02_04: (part02, vec!["59414"], 2018),
    }
}
