use crate::input::read_lines;
use std::{path::Path, collections::VecDeque};

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut monkeys = vec![];
    if let Ok(lines) = read_lines(path) {
        let mut lines = lines.flatten().skip(1);
        loop {
            let items_string = lines.next().unwrap();
            let operation_string = lines.next().unwrap();
            let test_string = lines.next().unwrap();
            let true_string = lines.next().unwrap();
            let false_string = lines.next().unwrap();
            let items = items_string.split(':').collect::<Vec<&str>>()[1].split(", ").map(|x| (x.trim()).parse::<i64>().unwrap()).collect::<VecDeque<i64>>();
            let operation_string = operation_string.split('=').collect::<Vec<&str>>()[1].trim().to_string();
            let operation_values = operation_string.split(' ').collect::<Vec<&str>>();
            let operation = match operation_values[1] {
                "+" => {
                    Operation::Add(get_operation_value(operation_values[0]), get_operation_value(operation_values[2]))
                }
                _ => {
                    Operation::Multiply(get_operation_value(operation_values[0]), get_operation_value(operation_values[2]))
                }
            };
            let test = test_string.split(' ').collect::<Vec<&str>>().last().unwrap().parse::<i64>().unwrap();
            let if_true = true_string.split(' ').collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap();
            let if_false = false_string.split(' ').collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap();
            let monkey = Monkey {
                items,
                operation,
                test,
                if_true,
                if_false
            };
            monkeys.push(monkey);
            if lines.next().is_some() {
                lines.next();
            } else {
                break;
            }
        }
    }

    println!("Part 1: {}", monkey_business(&mut monkeys.clone(),false)); 
    println!("Part 2: {}", monkey_business(&mut monkeys.clone(),true)); 
}

fn monkey_business(monkeys : &mut Vec<Monkey>, part2 : bool) -> u64 {
    let mut inspects = vec![0;monkeys.len()];
    let modulo : i64 = monkeys.iter().map(|x| x.test).product();
    let end = if part2 {
        10000
    } else {
        20
    };
    for _ in 0..end {
        for i in 0..monkeys.len() {
          while let Some(item) = monkeys[i].items.pop_front() {
                inspects[i] += 1;
                let item = monkeys[i].operation.apply(item);
                let item = if part2 {
                    item
                } else {
                    item / 3
                };
                let item = item % modulo;
                let next_monkey = if item % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[next_monkey].items.push_back(item);
            }
        }
    }
    inspects.sort_unstable();
    inspects.reverse();
    inspects[0] * inspects[1]
}

fn get_operation_value(str : &str) -> Option<i64> {
    let str = str.trim();
    str.parse::<i64>().ok()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Option<i64>, Option<i64>),
    Multiply(Option<i64>, Option<i64>)
}
impl Operation {
    pub fn apply(&self, old : i64) -> i64 {
        match self {
            Self::Add(a, b) => {
                a.unwrap_or(old) + b.unwrap_or(old)
            }
            Self::Multiply(a, b) => {
                a.unwrap_or(old) * b.unwrap_or(old)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    pub items : VecDeque<i64>,
    pub operation : Operation,
    pub test : i64,
    pub if_true : usize,
    pub if_false : usize
}