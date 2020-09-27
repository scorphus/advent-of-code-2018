/// # Day 07 (https://adventofcode.com/2018/day/7)
extern crate text_io;

use text_io::scan;

use std::collections::HashMap;
use std::collections::HashSet;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    Day::read_from(lines).part01()
}

struct Day {
    steps: HashSet<char>,
    needs: HashMap<char, HashSet<char>>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let (mut steps, mut needs) = (HashSet::new(), HashMap::new());
        for line in lines.iter() {
            let (pre, suc) = parse_instr(line.as_ref());
            steps.insert(pre);
            steps.insert(suc);
            needs.entry(suc).or_insert_with(HashSet::new).insert(pre);
        }
        Day { steps, needs }
    }

    fn part01(&self) -> String {
        let mut seen: HashSet<char> = HashSet::new();
        self.steps
            .iter()
            .map(|_| {
                let step = self
                    .steps
                    .iter()
                    .filter(|s| {
                        !seen.contains(s)
                            && match self.needs.get(s) {
                                Some(need) => need.is_subset(&seen),
                                None => true,
                            }
                    })
                    .min()
                    .expect("❌");
                seen.insert(*step);
                *step
            })
            .collect()
    }
}

fn parse_instr(string: &str) -> (char, char) {
    let mut r: (char, char) = (' ', ' ');
    scan!(
        string.bytes() => "Step {} must be finished before step {} can begin.", r.0, r.1
    );
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parts {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, instr, expected) = $values;
                    assert_eq!(method(&instr), expected);
                }
            )*
        }
    }

    test_parts! {
        test_part01_01: (part01, vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ], "CABDFE"),
    }
}
