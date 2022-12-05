use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let input_string = read_to_string(path).unwrap();
    let splits = input_string.split("\n\n").collect::<Vec<&str>>();
    let arrangement = splits[0];
    let mut lines = arrangement.split('\n').collect::<Vec<&str>>();
    let mut columns: Vec<VecDeque<char>> = vec![];
    lines.pop();
    for (_, l) in lines.iter().enumerate() {
        println!("{}", l);
        let chars = l.chars().collect::<Vec<char>>();
        for mut j in 0..chars.len() {
            if chars[j] == ' ' {
                while j + 1 < chars.len() && chars[j + 1] == ' ' {
                    j += 1;
                }
            } else if chars[j] != '[' && chars[j] != ']' {
                if columns.len() < j / 4 + 1 {
                    while columns.len() < j / 4 + 1 {
                        columns.push(VecDeque::new());
                    }
                }
                columns[j / 4].push_front(chars[j]);
            }
        }
    }
    let instructions: Vec<Vec<usize>> = splits[1]
        .split('\n')
        .map(|l| {
            l.split(' ')
                .filter_map(|x| x.to_string().trim().parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect();
    let columns2 = columns.clone();
    for instr in &instructions {
        for _ in 0..instr[0] {
            let c = columns[instr[1] - 1].pop_back();
            if let Some(c) = c {
                columns[instr[2] - 1].push_back(c);
            }
        }
    }

    // Part 2
    let mut columns = columns2;

    for instr in &instructions {
        let mut crates = vec![];
        for _ in 0..instr[0] {
            let c = columns[instr[1] - 1].pop_back();
            if let Some(c) = c {
                crates.push(c);
            }
        }
        crates.reverse();
        for c in crates {
            columns[instr[2] - 1].push_back(c);
        }
    }

    let mut result = vec![];
    for c in &mut columns {
        if let Some(v) = c.pop_back() {
            result.push(v);
        }
    }
    println!("{}", result.iter().collect::<String>());
}
