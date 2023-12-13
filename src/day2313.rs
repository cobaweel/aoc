use ndarray::Array2;

use crate::util::*;

// aoc_test!(part1, 231301, 405);

#[derive(Debug)]
struct Patterns(Vec<Pattern>);

#[derive(Debug)]
struct Pattern(Array2<bool>);

fn part1(patterns: Patterns) -> usize {
    println!("{patterns:?}");

    todo!()
}

impl FromStr for Patterns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let patterns = s
            .split("\n\n")
            .map(|pattern| {
                let cs = pattern
                    .trim()
                    .lines()
                    .map(|line| line.chars().map(|c| c == '#'));
                let a = aoc::array2(cs)?;
                Ok(Pattern(a))
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(Patterns(patterns))
    }
}
