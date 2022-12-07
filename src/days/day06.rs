use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let input_string = read_to_string(path).unwrap();
    for i in 0..input_string.len() - 4 {
        let substr = &input_string[i..i + 4];
        let subset = substr.chars().collect::<HashSet<char>>();
        if subset.len() == 4 {
            println!("Part 1: {}", i + 4);
            break;
        }
    }
    for i in 0..input_string.len() - 14 {
        let substr = &input_string[i..i + 14];
        let subset = substr.chars().collect::<HashSet<char>>();
        if subset.len() == 14 {
            println!("Part 2: {}", i + 14);
            break;
        }
    }
}
