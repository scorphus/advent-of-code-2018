use std::io;

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read error");
    line.trim_end().to_string()
}

pub fn string_to_i32(string: &str) -> i32 {
    string.parse::<i32>().expect("failed parsing i32")
}
