use std::io::{self, BufRead};

pub fn read_lines() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("read error"))
        .collect()
}

pub fn string_to_i32(string: &str) -> i32 {
    string.parse::<i32>().expect("failed parsing i32")
}
