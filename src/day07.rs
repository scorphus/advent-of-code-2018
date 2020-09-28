/// # Day 07 (https://adventofcode.com/2018/day/7)
extern crate text_io;

use text_io::scan;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg(not(test))]
const EXTRA_TIME_PER_STEP: i32 = 60;
#[cfg(not(test))]
const TOTAL_WORKERS: i32 = 5;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part02()
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
        let mut done: HashSet<char> = HashSet::new();
        self.steps
            .iter()
            .map(|_| {
                let step = self
                    .steps
                    .iter()
                    .filter(|s| {
                        !done.contains(s)
                            && match self.needs.get(s) {
                                Some(need) => need.is_subset(&done),
                                None => true,
                            }
                    })
                    .min()
                    .expect("❌");
                done.insert(*step);
                *step
            })
            .collect()
    }

    fn part02(&mut self) -> i32 {
        let mut second = 0;
        let mut active_workers = 0;
        let mut done: HashSet<char> = HashSet::new();
        let mut todo: HashMap<i32, HashSet<char>> = HashMap::new();
        let mut next_steps: VecDeque<char> = VecDeque::new();
        loop {
            for step in self.steps.iter() {
                if let Some(need) = self.needs.get(step) {
                    if !need.is_subset(&done) {
                        continue;
                    }
                }
                next_steps.push_back(*step);
            }
            self.steps.retain(|s| !next_steps.contains(s));
            while active_workers < TOTAL_WORKERS && !next_steps.is_empty() {
                if let Some(step) = next_steps.pop_front() {
                    active_workers += 1;
                    todo.entry(second + step_len(step))
                        .or_insert_with(HashSet::new)
                        .insert(step);
                }
            }
            if active_workers == 0 {
                break;
            }
            second = *todo.keys().min().expect("❌");
            let steps_done = todo.remove(&second).expect("❌");
            active_workers -= steps_done.len() as i32;
            done.extend(&steps_done);
        }
        second
    }
}

fn parse_instr(string: &str) -> (char, char) {
    let mut r: (char, char) = (' ', ' ');
    scan!(
        string.bytes() => "Step {} must be finished before step {} can begin.", r.0, r.1
    );
    r
}

fn step_len(step: char) -> i32 {
    EXTRA_TIME_PER_STEP + step as i32 - 'A' as i32 + 1
}

#[cfg(test)]
const EXTRA_TIME_PER_STEP: i32 = 0;
#[cfg(test)]
const TOTAL_WORKERS: i32 = 2;

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
        test_part02_01: (part02, vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ], 15),
    }
}
