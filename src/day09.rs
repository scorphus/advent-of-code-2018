/// Day 9 (https://adventofcode.com/2018/day/9)
extern crate text_io;

use text_io::scan;

use std::collections::VecDeque;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    players: usize,
    marble: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day::default();
        let line = lines.first().expect("❌").as_ref();
        scan!(line.bytes() => "{} players; last marble is worth {} points",
            day.players, day.marble);
        day
    }

    fn part01(&self) -> usize {
        let mut scores = vec![0; self.players];
        let mut circle = vec![0];
        let mut current = 0;
        for marble in 1..=self.marble {
            if marble % 23 == 0 {
                let remove_index = if current > 7 {
                    current - 7
                } else {
                    circle.len() + current - 7
                };
                let removed = circle.remove(remove_index);
                scores[marble % self.players] += marble + removed;
                current = remove_index % circle.len();
            } else {
                current = (current + 2) % circle.len();
                circle.insert(current, marble);
            }
        }
        scores.iter().cloned().max().expect("❌")
    }

    fn part02(&self) -> usize {
        let mut scores = vec![0; self.players];
        let mut circle: VecDeque<_> = [0].into();
        for marble in 1..=self.marble * 100 {
            if marble % 23 == 0 {
                circle.rotate_right(7);
                scores[marble % self.players] += marble + circle.pop_back().expect("❌");
                circle.rotate_left(1);
            } else {
                circle.rotate_left(1);
                circle.push_back(marble);
            }
        }
        scores.iter().cloned().max().expect("❌")
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
        test_part01_01: (part01, vec!["10 players; last marble is worth 1618 points"], 8317),
        test_part01_02: (part01, vec!["13 players; last marble is worth 7999 points"], 146373),
        test_part01_03: (part01, vec!["17 players; last marble is worth 1104 points"], 2764),
        test_part01_04: (part01, vec!["21 players; last marble is worth 6111 points"], 54718),
        test_part01_05: (part01, vec!["30 players; last marble is worth 5807 points"], 37305),
        test_part02_01: (part02, vec!["10 players; last marble is worth 1618 points"], 74765078),
        test_part02_02: (part02, vec!["13 players; last marble is worth 7999 points"], 1406506154),
        test_part02_03: (part02, vec!["17 players; last marble is worth 1104 points"], 20548882),
        test_part02_04: (part02, vec!["21 players; last marble is worth 6111 points"], 507583214),
        test_part02_05: (part02, vec!["30 players; last marble is worth 5807 points"], 320997431),
    }
}
