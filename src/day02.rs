/// # Day 02 (https://adventofcode.com/2018/day/2)
use crate::input;

use std::collections::HashMap;

struct Day {
    box_ids: Vec<String>,
}

impl Day {
    fn read() -> Self {
        let mut box_ids: Vec<String> = Vec::new();
        loop {
            let line = input::read_line();
            if line.is_empty() {
                break;
            }
            box_ids.push(line);
        }
        Day { box_ids }
    }

    fn part01(&self) -> i32 {
        let mut twice = 0;
        let mut thrice = 0;
        for id in &self.box_ids {
            let mut seen = (false, false);
            for (_, count) in count_letters(id) {
                if count == 2 && !seen.0 {
                    twice += 1;
                    seen.0 = true;
                } else if count == 3 && !seen.1 {
                    thrice += 1;
                    seen.1 = true;
                }
            }
        }
        twice * thrice
    }
}

fn count_letters(id: &str) -> HashMap<char, i32> {
    let mut letter_counts = HashMap::new();
    for letter in id.chars() {
        *letter_counts.entry(letter).or_insert(0) += 1;
    }
    letter_counts
}

pub fn part01() -> i32 {
    Day::read().part01()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_count_letters {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (id, expected) = $values;
                    let expected_map: HashMap<_, _> = expected.into_iter().collect();
                    assert_eq!(expected_map, count_letters(id));
                }
            )*
        }
    }

    test_count_letters! {
        test_count_letters_01: ("abcdef", vec![
            ('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1), ('f', 1)
        ]),
        test_count_letters_02: ("bababc", vec![
            ('a', 2), ('b', 3), ('c', 1)
        ]),
        test_count_letters_03: ("abbcde", vec![
            ('a', 1), ('b', 2), ('c', 1), ('d', 1), ('e', 1)
        ]),
        test_count_letters_04: ("abcccd", vec![
            ('a', 1), ('b', 1), ('c', 3), ('d', 1)
        ]),
        test_count_letters_05: ("aabcdd", vec![
            ('a', 2), ('b', 1), ('c', 1), ('d', 2)
        ]),
        test_count_letters_06: ("abcdee", vec![
            ('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 2)
        ]),
        test_count_letters_07: ("ababab", vec![
            ('a', 3), ('b', 3)
        ]),
    }

    macro_rules! test_parts {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, box_ids_str, expected) = $values;
                    let box_ids = box_ids_str.iter().map(|e| e.to_string()).collect();
                    assert_eq!(expected, method(&Day{ box_ids }));
                }
            )*
        }
    }

    test_parts! {
        test_part01_02: (Day::part01, vec![""], 0),
        test_part01_03: (Day::part01, vec!["aa"], 0),
        test_part01_04: (Day::part01, vec![
            "aa", "bb", "aaa", "bbb", "a", "ab", "abc",
        ], 4),
        test_part01_05: (Day::part01, vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ], 12),
    }
}
