use crate::input::read_lines;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut elves = Vec::new();
    let mut current_elf = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                if l.is_empty() {
                    elves.push(current_elf.clone());
                    current_elf = Vec::new();
                } else {
                    current_elf.push(l.parse::<i32>().unwrap())
                }
            }
        }
    }
    let mut elf_values = Vec::<(usize, i32)>::new();
    for (i, e) in elves.iter().enumerate() {
        let value = e.iter().sum();
        elf_values.push((i + 1, value));
    }
    elf_values.sort_by(|(_, b), (_, a)| a.partial_cmp(b).unwrap());
    println!("Part 1: {}", &elf_values[0].1);
    let sum = elf_values[0].1 + elf_values[1].1 + elf_values[2].1;
    println!("Part 2: {}", sum);
}
