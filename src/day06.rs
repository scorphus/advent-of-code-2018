/// Day 6 (https://adventofcode.com/2018/day/6)
use crate::input;

use std::collections::HashMap;

#[cfg(not(test))]
const TOTAL_DIST: i32 = 10_000;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part02()
}

struct Day {
    coords: Vec<(i32, i32)>,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let coords: Vec<(i32, i32)> = lines.iter().map(|l| parse_coords(l.as_ref())).collect();
        Day { coords }
    }

    fn part01(&self) -> i32 {
        let (mut grid, mut largest_area) = (HashMap::new(), 0);
        let (l, t, r, b) = self.find_limits();
        for x in l..r + 1 {
            for y in t..b + 1 {
                let (mut min_dist, mut unique, mut coords) = (i32::MAX, false, (0, 0));
                for &(cx, cy) in self.coords.iter() {
                    match (x - cx).abs() + (y - cy).abs() {
                        dist if dist < min_dist => {
                            min_dist = dist;
                            unique = true;
                            coords = (cx, cy);
                        }
                        dist if dist == min_dist => unique = false,
                        _ => {}
                    }
                }
                if unique {
                    if x == l || x == r || y == t || y == b {
                        grid.remove(&coords);
                        continue;
                    }
                    let area = grid.entry(coords).or_insert(0);
                    *area += 1;
                    if *area > largest_area {
                        largest_area = *area;
                    }
                }
            }
        }
        largest_area
    }

    fn part02(&self) -> i32 {
        let mut region_size = 0;
        let (l, t, r, b) = self.find_limits();
        for x in l..r + 1 {
            'y: for y in t..b + 1 {
                let mut dist_sum = 0;
                for &(cx, cy) in self.coords.iter() {
                    dist_sum += (x - cx).abs() + (y - cy).abs();
                    if dist_sum >= TOTAL_DIST {
                        continue 'y;
                    }
                }
                region_size += 1;
            }
        }
        region_size
    }

    fn find_limits(&self) -> (i32, i32, i32, i32) {
        let (mut l, mut t, mut r, mut b) = (i32::MAX, i32::MAX, 0, 0);
        for &(x, y) in self.coords.iter() {
            if x < l {
                l = x;
            }
            if y < t {
                t = y;
            }
            if x > r {
                r = x;
            }
            if y > b {
                b = y;
            }
        }
        (l, t, r, b)
    }
}

fn parse_coords(string: &str) -> (i32, i32) {
    let mut it = string.split(", ").map(|n| input::string_to_i32(n));
    (it.next().expect("❌"), it.next().expect("❌"))
}

#[cfg(test)]
const TOTAL_DIST: i32 = 32;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parts {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, coords, expected) = $values;
                    assert_eq!(method(&coords), expected);
                }
            )*
        }
    }

    test_parts! {
        test_part01_01: (part01, vec![
            "1, 1",
            "2, 2",
            "3, 3",
            "4, 4",
            "5, 5",
            "6, 6",
            "7, 7",
            "8, 8",
        ], 1),
        test_part01_02: (part01, vec![
            "1, 1",
            "3, 3",
            "5, 5",
            "7, 7",
        ], 7),
        test_part01_03: (part01, vec![
            "1, 1",
            "1, 6",
            "8, 3",
            "3, 4",
            "5, 5",
            "8, 9",
        ], 17),
        test_part02_01: (part02, vec![
            "1, 1",
            "2, 2",
            "3, 3",
            "4, 4",
            "5, 5",
            "6, 6",
            "7, 7",
            "8, 8",
        ], 0),
        test_part02_02: (part02, vec![
            "1, 1",
            "3, 3",
            "5, 5",
            "7, 7",
        ], 49),
        test_part02_03: (part02, vec![
            "1, 1",
            "1, 6",
            "8, 3",
            "3, 4",
            "5, 5",
            "8, 9",
        ], 16),
    }
}
