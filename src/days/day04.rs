use crate::input::read_lines;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(path) {
        let mut count = 0;
        let mut count2 = 0;
        for line in lines.flatten() {
            let ranges: Vec<i32> = line
                .split(&['-', ','][..])
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if (ranges[0] <= ranges[2] && ranges[1] >= ranges[3])
                || (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
            {
                count += 1;
            } else if (ranges[0] <= ranges[3] && ranges[1] >= ranges[2])
                || (ranges[0] >= ranges[3] && ranges[1] <= ranges[2])
            {
                count2 += 1;
            }
        }
        println!("Part 1: {}", count);
        println!("Part 2: {}", count + count2);
    }
}
