/// Day 4 (https://adventofcode.com/2018/day/4)
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
    records: Vec<Record>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Record {
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    guard_id: i32,
    event: Event,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Event {
    BeginShift,
    WakeUp,
    FallAsleep,
}

impl Day {
    fn read_from<T: AsRef<str>>(lines: &[T]) -> Self {
        let mut records: Vec<Record> = lines.iter().map(|l| parse_record(l.as_ref())).collect();
        records.sort();
        Day { records }
    }

    fn part01(&self) -> i32 {
        let stats = self.collect_stats();
        let (guard_id, (_, minutes)) = stats
            .iter()
            .max_by(|(_, a), (_, b)| a.0.cmp(&b.0))
            .expect("❌");
        let (minute, _) = minutes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .expect("❌");
        guard_id * minute as i32
    }

    fn part02(&self) -> i32 {
        let stats = self.collect_stats();
        let (guard_id, (_, minutes)) = stats
            .iter()
            .max_by(|(_, (_, a)), (_, (_, b))| {
                a.iter()
                    .max_by(|a, b| a.cmp(&b))
                    .cmp(&b.iter().max_by(|a, b| a.cmp(&b)))
            })
            .expect("❌");
        let (minute, _) = minutes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .expect("❌");
        guard_id * minute as i32
    }

    fn collect_stats(&self) -> HashMap<i32, (i32, Vec<i32>)> {
        let mut stats: HashMap<i32, (i32, Vec<i32>)> = HashMap::new();
        let mut guard_id = 0;
        let mut minute = 0;
        for record in self.records.iter() {
            match &record.event {
                Event::BeginShift => {
                    guard_id = record.guard_id;
                }
                Event::FallAsleep => {
                    minute = record.minute;
                }
                Event::WakeUp => {
                    let mut guard_stats = stats
                        .entry(guard_id)
                        .or_insert((0, (0..60).map(|_| 0).collect()));
                    guard_stats.0 += record.minute - minute;
                    for min in minute..record.minute {
                        guard_stats.1[min as usize] += 1;
                    }
                }
            }
        }
        stats
    }
}

fn parse_record(string: &str) -> Record {
    let mut r = Record {
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        guard_id: 0,
        event: Event::BeginShift,
    };
    if string.ends_with("begins shift") {
        scan!(string.bytes() => "[1518-{}-{} {}:{}] Guard #{} begins shift",
              r.month, r.day, r.hour, r.minute, r.guard_id
        );
    } else if string.ends_with("falls asleep") {
        scan!(string.bytes() => "[1518-{}-{} {}:{}] falls asleep",
              r.month, r.day, r.hour, r.minute
        );
        r.event = Event::FallAsleep;
    } else {
        scan!(string.bytes() => "[1518-{}-{} {}:{}] wakes up",
              r.month, r.day, r.hour, r.minute
        );
        r.event = Event::WakeUp;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_record {
        ($($name:ident: $values:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (record, expected) = $values;
                    assert_eq!(parse_record(record), expected);
                }
            )*
        }
    }

    test_parse_record! {
        test_parse_record_01: ("[1518-11-01 00:00] Guard #10 begins shift", Record{
            month: 11, day: 1, hour: 0, minute: 0, guard_id: 10, event: Event::BeginShift
        }),
        test_parse_record_02: ("[1518-11-01 23:58] Guard #99 begins shift", Record{
            month: 11, day: 1, hour: 23, minute: 58, guard_id: 99, event: Event::BeginShift
        }),
        test_parse_record_03: ("[1518-10-15 00:05] falls asleep", Record{
            month: 10, day: 15, hour: 0, minute: 5, guard_id: 0, event: Event::FallAsleep
        }),
        test_parse_record_04: ("[1518-11-02 00:40] falls asleep", Record{
            month: 11, day: 2, hour: 0, minute: 40, guard_id: 0, event: Event::FallAsleep
        }),
        test_parse_record_05: ("[1518-11-01 00:25] wakes up", Record{
            month: 11, day: 1, hour: 0, minute: 25, guard_id: 0, event: Event::WakeUp
        }),
        test_parse_record_06: ("[1518-11-04 00:46] wakes up", Record{
            month: 11, day: 4, hour: 0, minute: 46, guard_id: 0, event: Event::WakeUp
        }),
    }

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
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ], 240),
        test_part02_01: (part02, vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ], 4455),
    }
}
