#![allow(clippy::zero_prefixed_literal)]
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use std::env;

macro_rules! mod_days {($($day:tt)*) => (::paste::paste! {
    $(mod [<day$day>];)*
})}

mod_days! {01 02 03 04 05 06 07 08 09 10}

mod input;

fn main() {
    let day_part = env::args().nth(1).expect("missing <day>.<part> (e.g. 1.1)");
    solve_day(&day_part);
}

fn solve_day(day_part: &str) {
    let lines = input::read_lines();
    macro_rules! get_answer {($($day:tt)*) => (::paste::paste! {
        match day_part.replace(".", "").parse::<i32>() {
            $(
                Ok(dp) if dp == 10 * $day + 1 => [<day$day>]::part01(&lines).to_string(),
                Ok(dp) if dp == 10 * $day + 2 => [<day$day>]::part02(&lines).to_string(),
            )*
            _ => panic!("🤷"),
        }
    })}
    let answer = get_answer!(01 02 03 04 05 06 07 08 09 10);
    copy_to_clipboard(&answer);
    println!("Your answer is: {} (already copied to clipboard)", answer)
}

fn copy_to_clipboard(content: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("no clipboard");
    ctx.set_contents(content.to_string())
        .expect("failed copying to clipboard");
}
