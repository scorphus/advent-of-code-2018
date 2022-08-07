/// Day 12 (https://adventofcode.com/2018/day/12)
extern crate text_io;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    state: Vec<isize>,
    notes: Vec<isize>,
    prepend_count: isize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut lines_iter = lines.iter();
        let mut day = Day {
            state: lines_iter
                .next()
                .expect("❌")
                .as_ref()
                .split(' ')
                .nth(2)
                .expect("❌")
                .chars()
                .map(|c| (c == '#') as isize)
                .collect(),
            notes: vec![0; 32],
            prepend_count: 0,
        };
        lines_iter.next();
        for line in lines_iter {
            let mut line_iter = line.as_ref().split(" => ");
            let note_index = line_iter
                .next()
                .expect("❌")
                .replace('#', "1")
                .replace('.', "0");
            let note_value = line_iter
                .next()
                .expect("❌")
                .replace('#', "1")
                .replace('.', "0");
            let index = isize::from_str_radix(&note_index, 2).expect("❌");
            let value = isize::from_str_radix(&note_value, 2).expect("❌");
            day.notes[index as usize] = value;
        }
        day
    }

    fn part01(&mut self) -> isize {
        for _ in 0..20 {
            self.step();
        }
        self.sum_indexes()
    }

    fn part02(&mut self) -> isize {
        for _ in 0..500 {
            self.step();
        }
        self.sum_indexes() * 100_000_000
    }

    fn step(&mut self) {
        let mut new_state = Vec::new();
        let mut index = 0;
        self.prepend_count += 2;
        for pot in self.state.iter() {
            index = ((index << 1) & 31) | pot;
            let value = self.notes[index as usize];
            if !new_state.is_empty() || value == 1 {
                new_state.push(value);
            } else {
                self.prepend_count -= 1;
            }
        }
        for _ in 0..5 {
            index = (index << 1) & 31;
            new_state.push(self.notes[index as usize]);
        }
        self.state = new_state;
    }

    fn sum_indexes(&self) -> isize {
        self.state
            .iter()
            .enumerate()
            .map(|(i, pot)| (i as isize - self.prepend_count) * pot)
            .sum()
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
        test_part01_01: (part01, vec![
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ], 325),
        test_part02_01: (part02, vec![
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ], 937_400_000_000),
    }
}
