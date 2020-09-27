extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod input;

fn main() {
    let day_part = env::args().nth(1).expect("missing <day>.<part> (e.g. 1.1)");
    solve_day(&day_part);
}

fn solve_day(day_part: &str) {
    let lines = input::read_lines();
    let answer = match day_part {
        "1.1" => day01::part01(&lines).to_string(),
        "1.2" => day01::part02(&lines).to_string(),
        "2.1" => day02::part01(&lines).to_string(),
        "2.2" => day02::part02(&lines),
        "3.1" => day03::part01(&lines).to_string(),
        "3.2" => day03::part02(&lines).to_string(),
        "4.1" => day04::part01(&lines).to_string(),
        "4.2" => day04::part02(&lines).to_string(),
        "5.1" => day05::part01(&lines).to_string(),
        "5.2" => day05::part02(&lines).to_string(),
        "6.1" => day06::part01(&lines).to_string(),
        "6.2" => day06::part02(&lines).to_string(),
        "7.1" => day07::part01(&lines),
        _ => "🤷".to_string(),
    };
    copy_to_clipboard(&answer);
    println!("Your answer is: {} (already copied to clipboard)", answer)
}

fn copy_to_clipboard(content: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("no clipboard");
    ctx.set_contents(content.to_string())
        .expect("failed copying to clipboard");
}
