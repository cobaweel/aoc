use crate::util::*;

#[test]
fn test1() {
    aoc_parse_and_test(part1, 220101, 24000);
}

#[test]
fn test2() {
    aoc_parse_and_test(part1, 220100, 69206);
}

#[test]
fn test3() {
    aoc_parse_and_test(part2, 220101, 45000);
}

#[test]
fn test4() {
    aoc_parse_and_test(part2, 220100, 197400);
}

fn part1(input: Chart) -> u32 {
    sum_top(input, 1)
}

fn part2(input: Chart) -> u32 {
    sum_top(input, 3)
}

fn sum_top(input: Chart, n: usize) -> u32 {
    input
        .calories
        .into_iter()
        .map(|v| v.into_iter().sum::<u32>())
        .sorted_unstable()
        .rev()
        .take(n)
        .sum()
}

#[derive(From)]
struct Chart {
    calories: Vec<Vec<u32>>,
}

impl FromStr for Chart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;
        let calories = separated_list1(line_ending, u32);
        let calories = separated_list1(many1(line_ending).id(), calories);
        into(calories).anyhow(s)
    }
}
