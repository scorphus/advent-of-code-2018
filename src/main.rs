use std::env;

mod day01;
mod day02;
mod input;

fn solve_day(day_part: &str) {
    println!(
        "Your answer is: {}",
        match day_part {
            "1.1" => day01::part01().to_string(),
            "1.2" => day01::part02().to_string(),
            "2.1" => day02::part01().to_string(),
            "2.2" => day02::part02(),
            _ => "🤷".to_string(),
        }
    )
}

fn main() {
    let day_part = env::args().nth(1).expect("missing <day>.<part> (e.g. 1.1)");
    solve_day(&day_part);
}
