use crate::util::*;

aoc_test!(part1, 230101, 142);
aoc_test!(part1, 230100, 56108);
aoc_test!(part2, 230102, 281);
aoc_test!(part2, 230100, 55652);

fn part1(input: &str) -> u32 {
    input.lines().map(digits1).map(score).sum()
}

fn part2(input: &str) -> u32 {
    input.lines().map(digits2).map(score).sum()
}

fn score(digits: Vec<u32>) -> u32 {
    let x = digits.first().unwrap_or(&0);
    let y = digits.last().unwrap_or(&0);
    x * 10 + y
}

fn digits1(line: &str) -> Vec<u32> {
    let digits = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u32 - '0' as u32)
        .collect::<Vec<_>>();
    digits
}

fn digits2(line: &str) -> Vec<u32> {
    use std::{collections::BTreeMap, iter};

    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .into_iter()
    .flat_map(|(text, digit)| {
        line.match_indices(text)
            .map(|(index, _str)| index)
            .zip(iter::repeat(digit))
    })
    .collect::<BTreeMap<_, _>>()
    .into_values()
    .collect::<Vec<_>>()
}
