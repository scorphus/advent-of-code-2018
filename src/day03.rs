/// Day 3 (https://adventofcode.com/2018/day/3)
extern crate text_io;

use text_io::scan;

use std::collections::HashMap;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> i32 {
    Day::read_from(lines).part02()
}

struct Day {
    claims: Vec<Claim>,
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            claims: lines.iter().map(|l| parse_claim(l.as_ref())).collect(),
        }
    }

    fn part01(&self) -> i32 {
        let square_inches = self.collect_square_inches();
        let mut total = 0;
        for (_, count) in square_inches.iter() {
            if count > &1 {
                total += 1;
            }
        }
        total
    }

    fn collect_square_inches(&self) -> HashMap<(i32, i32), i32> {
        let mut square_inches = HashMap::new();
        for claim in self.claims.iter() {
            for w in claim.left..(claim.left + claim.width) {
                for h in claim.top..(claim.top + claim.height) {
                    *square_inches.entry((w, h)).or_insert(0) += 1;
                }
            }
        }
        square_inches
    }

    fn part02(&self) -> i32 {
        let square_inches = self.collect_square_inches();
        'claims: for claim in self.claims.iter() {
            for w in claim.left..(claim.left + claim.width) {
                for h in claim.top..(claim.top + claim.height) {
                    if square_inches[&(w, h)] > 1 {
                        continue 'claims;
                    }
                }
            }
            return claim.id;
        }
        -1
    }
}

fn parse_claim(string: &str) -> Claim {
    let mut c = Claim {
        id: 0,
        left: 0,
        top: 0,
        width: 0,
        height: 0,
    };
    scan!(string.bytes() => "#{} @ {},{}: {}x{}", c.id, c.left, c.top, c.width, c.height);
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_claim {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (claim, expected) = $values;
                    assert_eq!(parse_claim(claim), expected);
                }
            )*
        }
    }

    test_parse_claim! {
        test_parse_claim_01: ("#123 @ 3,2: 5x4", Claim{
            id: 123, left: 3, top: 2, width: 5, height: 4
        }),
        test_parse_claim_02: ("#1 @ 1,3: 4x4", Claim{
            id: 1, left: 1, top: 3, width: 4, height: 4
        }),
        test_parse_claim_03: ("#2 @ 3,1: 4x4", Claim{
            id: 2, left: 3, top: 1, width: 4, height: 4
        }),
        test_parse_claim_04: ("#3 @ 5,5: 2x2", Claim{
            id: 3, left: 5, top: 5, width: 2, height: 2
        }),
    }

    macro_rules! test_parts {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (method, claims, expected) = $values;
                    assert_eq!(method(&claims), expected);
                }
            )*
        }
    }

    test_parts! {
        test_part01_01: (part01, vec![
            "#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"
        ], 4),
        test_part01_02: (part01, vec![
            "#1 @ 0,0: 20x20", "#2 @ 20,20: 5x10", "#3 @ 25,30: 5x5"
        ], 0),
        test_part01_03: (part01, vec![
            "#1 @ 0,0: 20x20", "#2 @ 15,10: 5x10", "#3 @ 15,15: 5x5"
        ], 50),
        test_part01_04: (part01, vec![
            "#1 @ 0,0: 20x20", "#2 @ 15,10: 5x10", "#3 @ 10,10: 5x5"
        ], 75),
        test_part02_01: (part02, vec![
            "#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"
        ], 3),
        test_part02_02: (part02, vec![
            "#1 @ 0,0: 20x20", "#2 @ 20,20: 5x10", "#3 @ 25,30: 5x5"
        ], 1),
        test_part02_03: (part02, vec![
            "#1 @ 0,0: 20x20", "#2 @ 15,10: 5x10", "#3 @ 20,20: 5x5"
        ], 3),
        test_part02_04: (part02, vec![
            "#1 @ 0,0: 20x20", "#2 @ 20,20: 5x10", "#3 @ 20,20: 5x5"
        ], 1),
        test_part02_05: (part02, vec![
            "#1 @ 0,0: 20x20", "#2 @ 15,10: 5x10", "#3 @ 15,15: 5x5"
        ], -1),
        test_part02_06: (part02, vec![
            "#1 @ 0,0: 20x20", "#2 @ 15,10: 5x10", "#3 @ 10,10: 5x5"
        ], -1),
    }
}
