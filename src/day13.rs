/// Day 13 (https://adventofcode.com/2018/day/13)
extern crate text_io;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part01();
    format!("{},{}", x, y)
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> String {
    let (x, y) = Day::read_from(lines).part02();
    format!("{},{}", x, y)
}

#[derive(Debug, Default)]
struct Day {
    tracks: HashMap<Location, char>,
    carts: BinaryHeap<Cart>,
    occupied: HashSet<Location>,
}

type Location = (usize, usize);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Cart {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
    intersection_count: usize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        other.y.cmp(&self.y).then_with(|| other.x.cmp(&self.x))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day::default();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.as_ref().chars().enumerate() {
                if c != ' ' {
                    day.add_element(x, y, c);
                }
            }
        }
        day
    }

    fn add_element(&mut self, x: usize, y: usize, c: char) {
        let (vx, vy, new_c) = match c {
            '^' => (0, -1, '|'),
            'v' => (0, 1, '|'),
            '<' => (-1, 0, '-'),
            '>' => (1, 0, '-'),
            _ => (0, 0, c),
        };
        self.tracks.insert((x, y), new_c);
        if new_c != c {
            self.carts.push(Cart {
                x,
                y,
                vx,
                vy,
                ..Default::default()
            });
            self.occupied.insert((x, y));
        }
    }

    fn part01(&mut self) -> Location {
        loop {
            let mut new_carts = BinaryHeap::new();
            while let Some(mut cart) = self.carts.pop() {
                self.occupied.remove(&(cart.x, cart.y));
                cart.step();
                if self.occupied.contains(&(cart.x, cart.y)) {
                    return (cart.x, cart.y);
                }
                let track_section = self.tracks.get(&(cart.x, cart.y)).expect("❌");
                match track_section {
                    '|' | '-' => (),
                    &track_section => cart.turn(track_section),
                }
                self.occupied.insert((cart.x, cart.y));
                new_carts.push(cart);
            }
            self.carts = new_carts;
        }
    }

    fn part02(&mut self) -> Location {
        loop {
            let mut new_carts = BinaryHeap::new();
            while let Some(mut cart) = self.carts.pop() {
                if !self.occupied.contains(&(cart.x, cart.y)) {
                    continue;
                }
                self.occupied.remove(&(cart.x, cart.y));
                cart.step();
                if self.occupied.is_empty() {
                    return (cart.x, cart.y);
                }
                let track_section = self.tracks.get(&(cart.x, cart.y)).expect("❌");
                match track_section {
                    '|' | '-' => (),
                    &track_section => cart.turn(track_section),
                }
                if self.occupied.contains(&(cart.x, cart.y)) {
                    self.occupied.remove(&(cart.x, cart.y));
                    continue;
                }
                self.occupied.insert((cart.x, cart.y));
                new_carts.push(cart);
            }
            self.carts = new_carts;
        }
    }
}

impl Cart {
    fn step(&mut self) {
        self.x = (self.x as isize + self.vx) as usize;
        self.y = (self.y as isize + self.vy) as usize;
    }

    fn turn(&mut self, track_section: char) {
        match (track_section, self.vx) {
            ('/', 0) => self.turn_right(),
            ('/', _) => self.turn_left(),
            ('\\', 0) => self.turn_left(),
            ('\\', _) => self.turn_right(),
            _ => {
                match self.intersection_count % 3 {
                    0 => self.turn_left(),
                    2 => self.turn_right(),
                    _ => (),
                }
                self.intersection_count += 1;
            }
        }
    }

    fn turn_right(&mut self) {
        (self.vx, self.vy) = (-self.vy, self.vx);
    }

    fn turn_left(&mut self) {
        (self.vx, self.vy) = (self.vy, -self.vx);
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
            r"/->-\        ",
            r"|   |  /----\",
            r"| /-+--+-\  |",
            r"| | |  | v  |",
            r"\-+-/  \-+--/",
            r"  \------/   ",
        ], "7,3"),
        test_part02_01: (part02, vec![
            r"/>-<\  ",
            r"|   |  ",
            r"| /<+-\",
            r"| | | v",
            r"\>+</ |",
            r"  |   ^",
            r"  \<->/",
        ], "6,4"),
    }
}
