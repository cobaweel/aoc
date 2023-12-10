use crate::util::*;

#[test]
fn test1() {
    aoc_test(part1, 220301, 157);
    aoc_test(part1, 220300, 7785);
    aoc_test(part2, 220301, 70);
    aoc_test(part2, 220300, 2633);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(xx, yy)| intersect(&[xx, yy]))
        .map(priority)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(intersect)
        .map(priority)
        .sum()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn intersect(sets: &[&str]) -> char {
    use std::{collections::HashSet, iter};
    let mut common: HashSet<char> = iter::empty().collect();
    common.extend(sets[0].chars());
    for set in sets[1..].iter() {
        common.retain(|&c| set.contains(c));
    }
    *common.iter().next().expect("exactly 1 overlap")
}
