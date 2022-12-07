use crate::input::read_lines;
use std::collections::HashMap;
use std::path::Path;

pub fn run<P>(path: P)
where
    P: AsRef<Path>,
{
    let mut nodes = vec![];
    let mut current_node: usize = 0;
    nodes.push(TreeNode::Dir("/".to_string(), HashMap::new(), None, 0));
    if let Ok(lines) = read_lines(path) {
        for l in lines.flatten().skip(1) {
            let splits = l.split(' ').collect::<Vec<&str>>();
            match splits[0] {
                "$" => match splits[1] {
                    "cd" => match splits[2] {
                        ".." => {
                            if let Some(TreeNode::Dir(_, _, Some(parent), _)) =
                                nodes.get(current_node)
                            {
                                current_node = *parent;
                            }
                        }
                        _ => {
                            if let Some(TreeNode::Dir(_, v, _, _)) = nodes.get(current_node) {
                                current_node = v
                                    .get(&splits[2].to_string())
                                    .copied()
                                    .unwrap_or(current_node);
                            }
                        }
                    },
                    "ls" => {}
                    _ => {}
                },
                "dir" => {
                    let current_length = nodes.len();
                    let new_dir =
                        TreeNode::Dir(splits[1].to_string(), HashMap::new(), Some(current_node), 0);
                    nodes.push(new_dir);
                    if let Some(TreeNode::Dir(_, v, _, _)) = nodes.get_mut(current_node) {
                        v.insert(splits[1].to_string(), current_length);
                    }
                }
                _ => {
                    let current_length = nodes.len();
                    let size = splits[0].parse::<u64>().unwrap();
                    let name = splits[1].to_string();
                    nodes.push(TreeNode::File(name, size));
                    if let Some(TreeNode::Dir(_, v, _, _)) = nodes.get_mut(current_node) {
                        v.insert(splits[1].to_string(), current_length);
                    }
                }
            }
        }
    }
    calculate_sizes(&mut nodes, 0);
    let mut sum = 0;
    for n in &nodes {
        if let TreeNode::Dir(_, _, _, s) = n {
            if *s <= 100000 {
                sum += *s;
            }
        }
    }
    println!("Part 1: {}", sum);
    let disk_space = 70000000;
    let needed = 30000000;
    let available = if let TreeNode::Dir(_, _, _, s) = nodes[0] {
        disk_space - s
    } else {
        0
    };
    let mut smallest = disk_space;
    for n in &nodes {
        if let TreeNode::Dir(_, _, _, s) = n {
            if *s + available >= needed && *s < smallest {
                smallest = *s;
            }
        }
    }
    println!("Part 2: {}", smallest);
}

enum TreeNode {
    Dir(String, HashMap<String, usize>, Option<usize>, u64),
    File(String, u64),
}

fn calculate_sizes(nodes: &mut [TreeNode], node_idx: usize) -> u64 {
    if let Some(TreeNode::File(_, s)) = nodes.get(node_idx) {
        return *s;
    }
    let mut sum = 0;
    let mut children = vec![];
    if let Some(TreeNode::Dir(_, m, _, s)) = nodes.get(node_idx) {
        if *s != 0 {
            return *s;
        }
        for v in m.values() {
            children.push(*v);
        }
    }
    for c in children {
        sum += calculate_sizes(nodes, c);
    }

    if let Some(TreeNode::Dir(_, _, _, s)) = nodes.get_mut(node_idx) {
        *s = sum;
    }
    sum
}

#[allow(unused)]
fn print_tree(nodes: &[TreeNode], start_node: usize, depth: u64) -> String {
    let mut final_str = String::new();
    if let Some(TreeNode::Dir(n, m, _, s)) = nodes.get(start_node) {
        println!("{} - {}", n, depth);
        for _ in 0..depth * 2 {
            final_str.push(' ');
        }
        final_str.push_str(&format!("{} {}", n, s));
        final_str.push('\n');
        let mut children = m.iter().collect::<Vec<(&String, &usize)>>();
        children.sort();
        println!("{:?}", children);
        for (_, i) in children {
            if let Some(TreeNode::Dir(_, _, _, _)) = nodes.get(*i) {
                final_str.push_str(&print_tree(nodes, *i, depth + 1));
                final_str.push('\n');
            } else if let Some(TreeNode::File(n, s)) = nodes.get(*i) {
                for _ in 0..(depth + 1) * 2 {
                    final_str.push(' ');
                }
                final_str.push_str(&format!("{} {}", n, s));
                final_str.push('\n');
            }
        }
    }
    final_str.replace("\n\n", "\n")
}
