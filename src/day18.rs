/// Day 18 (https://adventofcode.com/2018/day/18)

const DELTAS: [(isize, isize); 8] = [
    (0, 1),   // north
    (1, 1),   // northeast
    (1, 0),   // east
    (1, -1),  // southeast
    (0, -1),  // south
    (-1, -1), // southwest
    (-1, 0),  // west
    (-1, 1),  // northwest
];

pub fn part01<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> usize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    area: Vec<Vec<char>>,
    size: usize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            area: lines.iter().map(|l| l.as_ref().chars().collect()).collect(),
            size: lines.len(),
        }
    }

    fn part01(&mut self) -> usize {
        for _ in 0..10 {
            self.tick();
        }
        let (mut wooded, mut lumberyards) = (0, 0);
        for row in self.area.iter() {
            for &c in row.iter() {
                if c == '#' {
                    lumberyards += 1;
                } else if c == '|' {
                    wooded += 1;
                }
            }
        }
        wooded * lumberyards
    }

    fn part02(&self) -> usize {
        0
    }

    fn tick(&mut self) {
        let mut acre_stats = vec![vec![(0, 0); self.size]; self.size];
        for y in 0..self.size {
            for x in 0..self.size {
                let acre = self.area[y][x];
                let (trees, lumber) = match acre {
                    '|' => (1, 0),
                    '#' => (0, 1),
                    _ => (0, 0),
                };
                for (dx, dy) in DELTAS {
                    let (nx, ny) = (x as isize + dx, y as isize + dy);
                    if nx < 0 || ny < 0 || nx >= self.size as isize || ny >= self.size as isize {
                        continue;
                    }
                    let stats = acre_stats[ny as usize][nx as usize];
                    acre_stats[ny as usize][nx as usize] = (stats.0 + trees, stats.1 + lumber);
                }
            }
        }
        let mut new_area = vec![vec!['.'; self.size]; self.size];
        for y in 0..self.size {
            for x in 0..self.size {
                new_area[y][x] = match (self.area[y][x], acre_stats[y][x]) {
                    ('.', (trees, _)) if trees >= 3 => '|',
                    ('|', (_, lumber)) if lumber >= 3 => '#',
                    ('#', (trees, lumber)) if trees >= 1 && lumber >= 1 => '#',
                    ('#', (_, _)) => '.',
                    (acre, _) => acre,
                };
            }
        }
        self.area = new_area;
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
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ], 1147),
        test_part02_01: (part02, vec![
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ], 0),
    }
}
