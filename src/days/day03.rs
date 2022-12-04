use crate::input::read_lines;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut backpacks: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                backpacks.push(l.chars().collect());
            }
        }
    }
    let mut sum = 0;
    for b in &backpacks {
        let c1 = &b[0..b.len() / 2];
        let c2 = &b[b.len() / 2..b.len()];
        for c in c1 {
            if c2.contains(c) {
                let value = calc_score(&c);
                sum += value;
                break;
            }
        }
    }
    let mut sum2 = 0;
    for i in 0..backpacks.len() / 3 {
        let b1 = &backpacks[i * 3];
        let b2 = &backpacks[i * 3 + 1];
        let b3 = &backpacks[i * 3 + 2];
        println!(
            "{}\n{}\n{}",
            b1.iter().collect::<String>(),
            b2.iter().collect::<String>(),
            b3.iter().collect::<String>()
        );
        for c in b1 {
            if b2.contains(c) && b3.contains(c) {
                sum2 += calc_score(c);
                break;
            }
        }
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}

fn calc_score(c: &char) -> u32 {
    if c.is_uppercase() {
        (*c as u32) - 38
    } else {
        (*c as u32) - 96
    }
}
