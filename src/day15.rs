/// Day 15 (https://adventofcode.com/2018/day/15)
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    game: Game,
}

#[derive(Clone, Debug, Default)]
struct Game {
    open_squares: HashSet<Position>,
    units: HashMap<Position, Unit>,
    order: BinaryHeap<Position>,
    elves: usize,
    goblins: usize,
    elf_attack_power: isize,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.y.cmp(&self.y).then_with(|| other.x.cmp(&self.x))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Unit = (char, isize);

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        Day {
            game: Game::read_from(lines),
        }
    }

    fn part01(&mut self) -> isize {
        self.game.elf_attack_power = 3;
        self.game.play()
    }

    fn part02(&self) -> isize {
        let mut min_outcome = 0;
        let mut min_power = 50;
        let (mut power_lo, mut power_hi) = (4, min_power);
        while power_lo < power_hi {
            let power = (power_lo + power_hi) / 2;
            let mut game = self.game.clone();
            game.elf_attack_power = power;
            let outcome = game.play();
            if game.elves == self.game.elves {
                power_hi = power;
                if power < min_power {
                    min_power = power;
                    min_outcome = outcome;
                }
            } else {
                power_lo = power + 1;
            }
        }
        min_outcome
    }
}

impl Game {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut game = Game::default();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.as_ref().chars().enumerate() {
                if c != '#' {
                    game.add_element(x, y, c);
                    if c == 'E' {
                        game.elves += 1;
                    } else if c == 'G' {
                        game.goblins += 1;
                    }
                }
            }
        }
        game
    }

    fn add_element(&mut self, x: usize, y: usize, c: char) {
        let position = Position {
            x: x as isize,
            y: y as isize,
        };
        self.open_squares.insert(position);
        if c != '.' {
            let unit = (c, 200);
            self.units.insert(position, unit);
            self.order.push(position);
        }
    }

    fn play(&mut self) -> isize {
        let mut rounds = 0;
        while self.elves > 0 && self.goblins > 0 {
            let mut new_order: BinaryHeap<Position> = BinaryHeap::new();
            while !self.order.is_empty() && self.elves > 0 && self.goblins > 0 {
                let position = self.order.pop().expect("❌");
                let &(kind, hp) = self.units.get(&position).expect("❌");
                if let Some((new_pos, enemy_pos, attack)) = self.turn(position, kind) {
                    if attack && self.attack(enemy_pos) {
                        new_order.retain(|p| p != &enemy_pos);
                    }
                    if new_pos != position {
                        self.units.remove(&position);
                        self.units.insert(new_pos, (kind, hp));
                    }
                    new_order.push(new_pos);
                } else {
                    new_order.push(position);
                }
            }
            if self.order.is_empty() {
                rounds += 1;
            }
            self.order = new_order;
        }
        rounds * self.units.values().map(|(_, hp)| hp).sum::<isize>()
    }

    fn turn(&self, position: Position, kind: char) -> Option<(Position, Position, bool)> {
        if let Some(enemy_pos) = self.aim(position, kind) {
            return Some((position, enemy_pos, true));
        }
        if let Some(new_pos) = self.step(position, kind) {
            if let Some(enemy_pos) = self.aim(new_pos, kind) {
                return Some((new_pos, enemy_pos, true));
            }
            return Some((new_pos, new_pos, false));
        }
        None
    }

    fn aim(&self, position: Position, kind: char) -> Option<Position> {
        let mut closest_pos: Option<Position> = None;
        let mut fewest_hp = isize::max_value();
        for (dx, dy) in DELTAS {
            let enemy_pos = Position {
                x: position.x + dx,
                y: position.y + dy,
            };
            if let Some((enemy_kind, enemy_hp)) = self.units.get(&enemy_pos) {
                if enemy_kind != &kind && enemy_hp < &fewest_hp {
                    closest_pos = Some(enemy_pos);
                    fewest_hp = *enemy_hp;
                }
            }
        }
        closest_pos
    }

    fn step(&self, position: Position, kind: char) -> Option<Position> {
        let mut candidates: BinaryHeap<Position> = BinaryHeap::new();
        let mut queue = VecDeque::new();
        let mut path = HashMap::new();
        queue.push_back((0, position));
        path.insert(position, position);
        let mut last_steps = 0;
        while !queue.is_empty() {
            let (steps, cur_pos) = queue.pop_front().expect("❌");
            if steps > last_steps && !candidates.is_empty() {
                break;
            }
            for (dx, dy) in DELTAS {
                let new_pos = Position {
                    x: cur_pos.x + dx,
                    y: cur_pos.y + dy,
                };
                if !self.open_squares.contains(&new_pos) || path.contains_key(&new_pos) {
                    continue;
                }
                if let Some((enemy_kind, _)) = self.units.get(&new_pos) {
                    if enemy_kind != &kind {
                        candidates.push(cur_pos);
                    }
                    continue;
                }
                queue.push_back((steps + 1, new_pos));
                path.insert(new_pos, cur_pos);
            }
            last_steps = steps;
        }
        if let Some(mut move_pos) = candidates.pop() {
            while let Some(&prev_pos) = path.get(&move_pos) {
                if prev_pos == position {
                    return Some(move_pos);
                }
                move_pos = prev_pos;
            }
        }
        None
    }

    fn attack(&mut self, enemy_pos: Position) -> bool {
        if let Some(&(enemy_kind, enemy_hp)) = self.units.get(&enemy_pos) {
            let mut power = 3;
            if enemy_kind == 'G' {
                power = self.elf_attack_power;
            }
            if enemy_hp > power {
                self.units.insert(enemy_pos, (enemy_kind, enemy_hp - power));
            } else {
                if enemy_kind == 'E' {
                    self.elves -= 1;
                } else {
                    self.goblins -= 1;
                }
                self.units.remove(&enemy_pos);
                self.order.retain(|pos| *pos != enemy_pos);
                return true;
            }
        }
        false
    }
}

const DELTAS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)]; // up, left, right, down

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
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ], 27_730),
        test_part01_02: (part01, vec![
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
        ], 36_334),
        test_part01_03: (part01, vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ], 39_514),
        test_part01_04: (part01, vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ], 27_755),
        test_part01_05: (part01, vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ], 28_944),
        test_part01_06: (part01, vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ], 18_740),
        test_part02_01: (part02, vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ], 4_988),
        test_part02_03: (part02, vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ], 31_284),
        test_part02_04: (part02, vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ], 3_478),
        test_part02_05: (part02, vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ], 6_474),
        test_part02_06: (part02, vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ], 1_140),
    }
}
