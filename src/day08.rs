/// Day 8 (https://adventofcode.com/2018/day/8)
extern crate text_io;

use crate::input;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part02()
}

struct Day {
    root: Node,
}

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    meta: Vec<i32>,
    size: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let numbers: Vec<i32> = lines
            .first()
            .expect("❌")
            .as_ref()
            .split(' ')
            .map(|s| input::string_to_i32(s))
            .collect();
        Day {
            root: parse_nodes(&numbers).expect("❌"),
        }
    }

    fn part01(&self) -> i32 {
        self.root.all_meta_sum()
    }

    fn part02(&self) -> i32 {
        self.root.value()
    }
}

impl Node {
    fn all_meta_sum(&self) -> i32 {
        self.meta_sum() + self.children.iter().map(|c| c.all_meta_sum()).sum::<i32>()
    }

    fn meta_sum(&self) -> i32 {
        self.meta.iter().sum()
    }

    fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.meta.iter().sum()
        } else {
            self.meta
                .iter()
                .map(|&m| {
                    if let Some(node) = self.children.get(m as usize - 1) {
                        node.value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

fn parse_nodes(numbers: &[i32]) -> Option<Node> {
    let mut node = Node {
        size: 2,
        ..Node::default()
    };
    for _ in 0..numbers[0] {
        let child = parse_nodes(&numbers[node.size..])?;
        node.size += child.size;
        node.children.push(child);
    }
    for _ in 0..numbers[1] {
        let meta = numbers.get(node.size)?;
        node.meta.push(*meta);
        node.size += 1;
    }
    Some(node)
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
        test_part01_01: (part01, vec!["2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"], 138),
        test_part02_01: (part02, vec!["2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"], 66),
    }
}
