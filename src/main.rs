mod days;
mod input;

use chrono::Datelike;
use days::*;
use std::env;

fn run_day(day : u32, info: &Option<&str>, append : &Option<String>) -> bool {

    println!("Day {}: Running {}...\n", day, info.unwrap_or(""));
    let file_path = create_file_path(day, append);

    match day {
        1 => {
            day01::run(&file_path);
        }
        2 => {
            day02::run(&file_path);
        }
        3 => {
            day03::run(&file_path);
        }
        4 => {
            day04::run(&file_path);
        }
        5 => {
            day05::run(&file_path);
        }
        6 => {
            day06::run(&file_path);
        }
        7 => {
            day07::run(&file_path);
        }
        8 => {
            day08::run(&file_path);
        }
        9 => {
            day09::run(&file_path);
        }
        10 => {
            day10::run(&file_path);
        }
        11 => {
            day11::run(&file_path);
        }
        _ => {
            return false;
        }
    }
    true
}
pub fn main() {
    let mut day = chrono::offset::Utc::now().date_naive().day();
    let mut append = None;
    let mut info = None;

    let args: Vec<String> = env::args().collect();
    let mut all = false;
    if args.len() > 1 {
        let args = &args[1..];
        for a in args {
            if let Ok(n) = a.parse::<i64>() {
                day = n as u32;
            } else {
                match a.as_str() {
                    "-e" | "example" => {
                        info = Some("Example ");
                        append = Some("_example".to_string());
                    }
                    "-a" | "all" => {
                        all = true;
                    }
                    _ => info = Some("Custom Input "),
                }
            }
        }
    }
    if ! all {

        run_day(day, &info, &append);
    } else {
        let mut day = 1;
        while run_day(day, &info, &append) {
            day += 1;
            println!();
        }
    }

}

fn create_file_path(day: u32, append: &Option<String>) -> String {
    let day = if day < 10 {
        format!("0{}", day)
    } else {
        day.to_string()
    };
    if let Some(a) = append {
        format!("./inputs/{}{}", day, a)
    } else {
        format!("./inputs/{}", day)
    }
}
