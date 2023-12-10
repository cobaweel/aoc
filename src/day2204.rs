use crate::util::*;

aoc_test!(part1, 220401, 2);
aoc_test!(part1, 220400, 448);
aoc_test!(part2, 220401, 4);
aoc_test!(part2, 220400, 794);

#[derive(Debug)]
struct Assignment {
    lo1: u32,
    hi1: u32,
    lo2: u32,
    hi2: u32,
}

fn sorted(xs: &[impl PartialOrd]) -> bool {
    xs.windows(2).all(|w| w[0] <= w[1])
}

fn part1(input: &str) -> u32 {
    input.lines().map(parse).filter(has_inclusion).count() as u32
}

fn part2(input: &str) -> u32 {
    input.lines().map(parse).filter(has_overlap).count() as u32
}

fn has_inclusion(assignment: &Assignment) -> bool {
    let Assignment { lo1, hi1, lo2, hi2 } = assignment;
    sorted(&[lo1, lo2, hi2, hi1]) || sorted(&[lo2, lo1, hi1, hi2])
}

fn has_overlap(assignment: &Assignment) -> bool {
    let Assignment { lo1, hi1, lo2, hi2 } = assignment;
    sorted(&[lo1, lo2, hi2, hi1])
        || sorted(&[lo1, lo2, hi1, hi2])
        || sorted(&[lo2, lo1, hi1, hi2])
        || sorted(&[lo2, lo1, hi2, hi1])
}

fn parse(line: &str) -> Assignment {
    let numbers = line
        .split(|c| c == ',' || c == '-')
        .map(|s| s.parse::<u32>().expect("number"))
        .collect::<Vec<_>>();
    Assignment {
        lo1: numbers[0],
        hi1: numbers[1],
        lo2: numbers[2],
        hi2: numbers[3],
    }
}
