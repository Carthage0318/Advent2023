use advent_2023::{utils, PromptDayErr};
use std::process;

fn main() {
    utils::advent_header();

    let day = loop {
        match advent_2023::prompt_day_number() {
            Ok(day) => {
                if advent_2023::is_valid_day(day) {
                    break day;
                } else {
                    eprintln!("Invalid day '{day}'")
                }
            }
            Err(PromptDayErr::ParseDay(input)) => {
                eprintln!("Invalid day '{input}'");
                continue;
            }
            Err(e) => {
                println!("{e}");
                process::exit(1);
            }
        }
    };

    let day_fn = advent_2023::get_day_fn(day).unwrap_or_else(|| {
        eprintln!("Day {day} isn't implemented yet!");
        process::exit(3);
    });

    let input_file = advent_2023::get_input_file(day).unwrap_or_else(|e| {
        eprintln!("Error opening file for day {day}:\n{e}");
        process::exit(2);
    });

    utils::day_header(day);
    day_fn(input_file).unwrap_or_else(|e| eprintln!("Error executing day {day}:\n{e}"));
}
