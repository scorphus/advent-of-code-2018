/// Day 10 (https://adventofcode.com/2018/day/10)
extern crate text_io;

use text_io::scan;

use std::collections::HashSet;

pub fn part01<T: AsRef<str>>(lines: &[T]) -> String {
    Day::read_from(lines).part01()
}

pub fn part02<T: AsRef<str>>(lines: &[T]) -> isize {
    Day::read_from(lines).part02()
}

#[derive(Debug, Default)]
struct Day {
    points: Vec<Point>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

#[derive(Debug, Default)]
struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut day = Day {
            points: lines.iter().map(|l| parse_points(l.as_ref())).collect(),
            ..Default::default()
        };
        day.update_max_min();
        day
    }

    fn update_max_min(&mut self) {
        self.min_x = isize::MAX;
        self.max_x = isize::MIN;
        self.min_y = isize::MAX;
        self.max_y = isize::MIN;
        for point in &self.points {
            if point.x < self.min_x {
                self.min_x = point.x;
            }
            if point.x > self.max_x {
                self.max_x = point.x;
            }
            if point.y < self.min_y {
                self.min_y = point.y;
            }
            if point.y > self.max_y {
                self.max_y = point.y;
            }
        }
    }

    fn part01(&mut self) -> String {
        let mut min_x_dist = isize::MAX;
        while self.max_x - self.min_x <= min_x_dist {
            min_x_dist = self.max_x - self.min_x;
            self.step();
            self.update_max_min();
        }
        self.step_back();
        self.update_max_min();
        self.draw()
    }

    fn part02(&mut self) -> isize {
        let mut seconds = 0;
        let mut min_x_dist = isize::MAX;
        while self.max_x - self.min_x <= min_x_dist {
            min_x_dist = self.max_x - self.min_x;
            self.step();
            self.update_max_min();
            seconds += 1;
        }
        seconds - 1
    }

    fn draw(&self) -> String {
        let mut set: HashSet<(isize, isize)> = HashSet::new();
        for point in &self.points {
            set.insert((point.x, point.y));
        }
        let mut s = String::new();
        s.push('\n');
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if set.contains(&(x, y)) {
                    s.push('#');
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }
        s
    }

    fn step(&mut self) {
        for point in &mut self.points {
            point.x += point.vx;
            point.y += point.vy;
        }
    }

    fn step_back(&mut self) {
        for point in &mut self.points {
            point.x -= point.vx;
            point.y -= point.vy;
        }
    }
}

fn parse_points(string: &str) -> Point {
    let mut p = Point {
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
    };
    let string = string.replace(' ', "");
    scan!(string.bytes() => "position=<{},{}>velocity=<{},{}>", p.x, p.y, p.vx, p.vy);
    p
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

    #[rustfmt::skip]
    test_parts! {
        test_part01_01: (part01, vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>",
        ], "
#   #  ###\n\
#   #   # \n\
#   #   # \n\
#####   # \n\
#   #   # \n\
#   #   # \n\
#   #   # \n\
#   #  ###\n"),

        test_part02_01: (part02, vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>",
        ], 3),
    }
}
