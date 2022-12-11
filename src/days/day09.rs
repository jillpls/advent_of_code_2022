use crate::input::read_lines;
use std::path::Path;
use std::collections::HashMap;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut instructions = vec![];
    if let Ok(lines) = read_lines(path) {
        for l in lines.flatten() {
            let split = l.split(' ').collect::<Vec<&str>>();
            instructions.push((split[0].chars().next().unwrap(), split[1].parse::<i32>().unwrap()));
        }
    }
    let mut current_ver = 0;
    let mut max_ver = 0;
    let mut min_ver = 0;
    let mut current_hor = 0;
    let mut max_hor= 0;
    let mut min_hor = 0;
    for (d, amount) in instructions.iter() {
            
        match d {
            'R' => {
                current_hor += amount; 
            }
            'L' => {
                current_hor -= amount; 
            }
            'U' => {
                current_ver += amount;
            }
            'D' => {
                current_ver -= amount;
            }
            _ => {}
        }
        if current_ver > max_ver {
            max_ver = current_ver;
        }
        if current_ver < min_ver {
            min_ver = current_ver;
        }
        if current_hor > max_hor {
            max_hor = current_hor;
        }
        if current_hor < min_hor { 
            min_hor = current_hor;
        }
    }

    let mut blank_map = vec![vec!['.'; (max_hor+min_hor.abs()) as usize + 1];(max_ver+min_ver.abs()) as usize + 1];
    let start = (-min_hor, -min_ver );
    blank_map[start.1 as usize][start.0 as usize] = 's';

    let mut instruction_info = HashMap::new();
    let mut paths = vec![vec![(start.0, start.1)];10];
    for (d, amount) in instructions.iter() {
        let (x, y) = paths[0].last().copied().unwrap();
        instruction_info.insert(paths[0].len(), format!("== {} {} ==", d, amount));
        for i in 1..=*amount {
            match *d {
                'R' => {
                    paths[0].push((x+i, y));
                }
                'L' => {
                    paths[0].push((x-i, y));
                }
                'U' => {
                    paths[0].push((x, y+i));
                }
                'D' => {
                    paths[0].push((x, y-i));
                }
                _ => {}
            }
            for k in 0..(paths.len()-1) {
                let (x_t, y_t) = paths[k+1].last().copied().unwrap();
                let (x_h, y_h) = paths[k].last().copied().unwrap();
                let x_diff = x_h - x_t;
                let y_diff = y_h - y_t;
                if x_diff.abs() >= 2 && y_diff.abs() >= 2 {
                    paths[k+1].push((x_h - x_diff.signum(), y_h - y_diff.signum())); 
                } else if x_diff.abs() >= 2 {
                    paths[k+1].push((x_h - x_diff.signum(), y_h));
                } else if y_diff.abs() >= 2 {
                    paths[k+1].push((x_h, y_h - y_diff.signum()));
                } else {
                    paths[k+1].push((x_t, y_t));
                }
            }
        }
    }
    let mut visited_map = blank_map.clone();
    let mut visited = 0;
    for (x, y) in &paths[1] {
        if visited_map[*y as usize][*x as usize] != '#' {
            visited_map[*y as usize][*x as usize] = '#';
            visited += 1;
        }
    }
    println!("Part 1: {}", visited);
    visited_map = blank_map.clone();
    visited = 0;
    for (x, y) in paths.last().unwrap() {
        if visited_map[*y as usize][*x as usize] != '#' {
            visited_map[*y as usize][*x as usize] = '#';
            visited += 1;
        }
    }
    println!("Part 2: {}", visited);

    if max_ver > 50 || max_hor > 50 {
        return;
    }

    for i in 0..paths[0].len() {
        let (x_h, y_h) = paths[0][i];
        let mut current_map = blank_map.clone();
        for k in (1..paths.len()).rev() {
            let (x_t, y_t) = paths[k][i];
            current_map[y_t as usize][x_t as usize] = char::from_digit(k as u32, 10).unwrap();
        }
        current_map[y_h as usize][x_h as usize] = 'H';
        if let Some(instr) = instruction_info.get(&i) {
            println!("{}", instr);
        }
        for r in current_map.iter().rev() {
            println!("{}", r.iter().collect::<String>());
        }
        println!()
    }


}