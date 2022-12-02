use crate::input::read_lines;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut instructions = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let splits: Vec<&str> = l.split(" ").collect();

                instructions.push((splits[0].to_string(), splits[1].to_string()));
            }
        }
    }
    let mut score_1 = 0;
    let mut score_2 = 0;
    for (a, b) in instructions {
        let p1 = simplify_action(&a);
        let p2 = simplify_action(&b);
        let score = get_score(p1, p2);
        score_1 += score;
        let score = get_instructed_score(p1, p2);
        score_2 += score;
    }
    println!("Part 1: {}", score_1);
    println!("Part 2: {}", score_2);
}

fn simplify_action(a: &str) -> i32 {
    match a {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => 0,
    }
}

fn get_score(p1: i32, p2: i32) -> i32 {
    let base_score = p2 + 1;
    let score = if p1 == p2 {
        base_score + 3
    } else if ((p1 + 1) % 3) == p2 {
        base_score + 6
    } else {
        base_score
    };
    score
}

fn get_instructed_score(p1: i32, p2: i32) -> i32 {
    let score = match p2 {
        0 => {
            if p1 == 0 {
                3
            } else {
                p1
            }
        }
        1 => {
            p1 + 4
        }
        2 => {
            (p1 + 1) % 3 + 7
        }
        _ => {
            0
        }
    };
    score
}
