use ndarray::{s, Array2, ArrayView2};

use crate::util::*;

aoc_test!(part1, 231301, 405);
aoc_test!(part1, 231300, 33735);
aoc_test!(part2, 231301, 400);
aoc_test!(part2, 231300, 38063);

#[derive(Debug)]
struct Patterns(Vec<Pattern>);

#[derive(Debug)]
struct Pattern(Array2<bool>);

fn part1(patterns: Patterns) -> usize {
    patterns.summary(0)
}

fn part2(patterns: Patterns) -> usize {
    patterns.summary(1)
}

impl Patterns {
    fn summary(&self, expected_flaws: usize) -> usize {
        self.0
            .iter()
            .map(|pattern| pattern.summary(expected_flaws))
            .sum()
    }
}

impl Pattern {
    fn summary(&self, expected_flaws: usize) -> usize {
        let array = &self.0;
        None.or_else(|| Self::col_summary(array.view(), expected_flaws))
            .or_else(|| Self::row_summary(array.view(), expected_flaws))
            .unwrap_or(0)
    }

    fn row_summary(array: ArrayView2<bool>, expected_flaws: usize) -> Option<usize> {
        Self::find_reflection(array, expected_flaws).map(|i| i * 100)
    }

    fn col_summary(array: ArrayView2<bool>, expected_flaws: usize) -> Option<usize> {
        Self::find_reflection(array.t(), expected_flaws)
    }

    fn find_reflection(array: ArrayView2<bool>, expected_flaws: usize) -> Option<usize> {
        let (rows, _) = array.dim();
        (1..rows).find(|&i| {
            let w = min(i, rows - i);
            let top = array.slice(s![i-w..i;-1, ..]);
            let bot = array.slice(s![i..i + w, ..]);
            let flaws = ndarray::Zip::from(top)
                .and(bot)
                .fold(0, |s, p, q| if p == q { s } else { s + 1 });
            flaws == expected_flaws
        })
    }
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
                Ok(Pattern(aoc::array2(cs)?))
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(Patterns(patterns))
    }
}
