use crate::input::read_lines;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut rows = vec![];
    if let Ok(lines) = read_lines(path) {
        for l in lines.flatten() {
            rows.push(
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>(),
            )
        }
    }
    let mut visible = vec![vec![0; rows[0].len()]; rows.len()];
    for i in 0..rows.len() {
        let mut max: i32 = -1;
        for j in 0..rows[0].len() {
            if rows[i][j] > max {
                max = rows[i][j];
                visible[i][j] = 1;
            }
        }
        let mut max = -1;
        for j in (0..rows[0].len()).rev() {
            if rows[i][j] > max {
                max = rows[i][j];
                visible[i][j] = 1;
            }
        }
    }
    for j in 0..rows[0].len() {
        let mut max: i32 = -1;
        for i in 0..rows.len() {
            if rows[i][j] > max {
                max = rows[i][j];
                visible[i][j] = 1;
            }
        }
        let mut max = -1;
        for i in (0..rows.len()).rev() {
            if rows[i][j] > max {
                max = rows[i][j];
                visible[i][j] = 1;
            }
        }
    }
    let mut visible_count = 0;
    for i in 0..visible.len() {
        for j in 0..visible[0].len() {
            if visible[i][j] == 1 {
                visible_count += 1;
            }
        }
    }
    println!("Part 1: {}", visible_count);
    let mut scenic_scores = vec![vec![0; rows[0].len()]; rows.len()];
    let mut max_scenic_score = 0;
    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            let mut scenic_score = 1;
            for dir in dirs {
                let mut inner_scenic_score = 0;
                for k in 1..(rows.len() as i32) {
                    let pos = ((i as i32) + dir.0 * k, (j as i32) + dir.1 * k);
                    if pos.0 < 0 || pos.1 < 0 {
                        break;
                    }
                    let pos = (pos.0 as usize, pos.1 as usize);
                    if let Some(v) = rows.get(pos.0).unwrap_or(&vec![]).get(pos.1) {
                        inner_scenic_score += 1;
                        if *v >= rows[i][j] {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                scenic_score *= inner_scenic_score;
            }
            scenic_scores[i][j] = scenic_score;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("Part 2: {}", max_scenic_score);
}
