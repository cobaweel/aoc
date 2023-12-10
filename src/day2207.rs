use itertools::Itertools;
use std::{collections::HashMap, iter};

#[test]
fn test() {
    use crate::util::test;
    test(part1, 220701, 95437);
    test(part1, 220700, 1555642);
    test(part2, 220701, 24933642);
    test(part2, 220700, 5974547);
}

fn part1(input: &str) -> usize {
    let size_by_name = get_size_by_name(input);
    size_by_name.values().filter(|v| v <= &&100000).sum()
}

fn part2(input: &str) -> usize {
    let size_by_name = get_size_by_name(input);
    let disk_size = 70000000_usize;
    let used = *size_by_name.get("").unwrap_or(&0);
    let free = disk_size - used;
    let required = 30000000;
    let overage = required - free;
    size_by_name
        .into_values()
        .sorted()
        .find(|v| *v >= overage)
        .unwrap_or(0)
}

fn get_size_by_name(input: &str) -> HashMap<String, usize> {
    let mut cwd: Vec<String> = iter::empty().collect();
    let mut size_by_name: HashMap<String, usize> = iter::empty().collect();
    let lines = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_vec());
    for line in lines {
        match line.as_slice() {
            ["$", "ls"] => {}
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", "/"] => {
                cwd.clear();
                cwd.push("".to_string())
            }
            ["$", "cd", name] => {
                let path = format!(
                    "{}/{}",
                    cwd.last().cloned().unwrap_or_else(String::new),
                    name
                );
                cwd.push(path);
            }
            ["dir", _] => {}
            [size, _] => {
                let size: usize = size.parse().unwrap_or(0);
                for component in cwd.iter() {
                    *size_by_name.entry(component.to_string()).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    size_by_name
}
