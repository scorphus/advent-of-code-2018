use std::fs;
use std::io::{self, BufRead};

pub fn read_lines() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("read error"))
        .collect()
}

pub fn read_lines_from_input(path: &str) -> Vec<String> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    data.lines().map(|l| l.to_string()).collect()
}

pub fn string_to_i32(string: &str) -> i32 {
    string.parse::<i32>().expect("failed parsing i32")
}
