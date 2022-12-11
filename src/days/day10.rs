use crate::input::read_lines;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut instructions = vec![];
    if let Ok(lines) = read_lines(path) {
        for l in lines.flatten() {
            let split = l.split(' ').collect::<Vec<&str>>();
            let v = split.get(1).map(|x| x.parse::<i32>().unwrap() );
            instructions.push((split[0].to_string(), v));
        }
    }
    let mut sum = 0;
    let mut next_cycle = 20;
    let mut x = 1;
    let mut c = 0;
    let mut cycled_instructions = vec![];
    for (i, v) in instructions.iter() {
        cycled_instructions.push(("noop".to_string(), None));
        if i.as_str() != "noop" {
            cycled_instructions.push((i.clone(), *v));
        } 
    }
    for (i, v) in instructions {
        if i.as_str() == "noop" {
            c +=1;
        } else {
            c +=2;
        }

        if c >= next_cycle && next_cycle <= 220 {
            sum += next_cycle*x;
            next_cycle += 40;
        }

        if i.as_str() == "addx" {
            let v = v.unwrap();
            x += v;
        }
    }
    println!("Part 1: {}", sum);

    x = 1;

    println!("Part 2:");
    for c in 1..=240 {
        let pos = ((c-1) % 40) as i32;
        if (pos-x).abs() <= 1 {
            print!("#");
        } else {
            print!(" ");
        }
        if c % 40 == 0 {
            println!();
        }
        let (i, v) = &cycled_instructions[((c-1) as usize)];
        if i.as_str() == "noop" {
            continue;
        }
        x += v.unwrap();
    }

  
}