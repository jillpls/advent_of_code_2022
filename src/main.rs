mod days;
mod input;

use chrono::Datelike;
use days::*;
use std::env;

pub fn main() -> () {
    let mut day = chrono::offset::Utc::now().date_naive().day();
    let mut append = None;
    let mut info = None;

    let args: Vec<String> = env::args().collect();
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
                    _ => { info = Some("Custom Input ") }
                }
            }
        }
    }

    println!("Day {}: Running {}...\n", day, info.unwrap_or(""));

    let file_path = create_file_path(day, append);

    match day {
        1 => {
            day01::run(&file_path);
        }
        _ => {
            if day > 25 {
                panic!("There is no day {}.", day);
            } else {
                panic!("It is not day {} yet.", day);
            }
        }
    }
}

fn create_file_path(day: u32, append: Option<String>) -> String {
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
