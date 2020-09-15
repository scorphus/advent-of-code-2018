use std::env;

mod day01;
mod input;

fn solve_day(day_part: &str) {
    match day_part {
        "1.1" => println!("Your answer is: {:?}", day01::part01()),
        _ => println!("🤷🏻‍♂️"),
    }
}

fn main() {
    let day_part = env::args().nth(1).expect("missing <day>.<part> (e.g. 1.1)");
    solve_day(&day_part);
}
