#[test]
fn test() {
    use crate::util::test;
    test(part1, 220101, 24000);
    test(part2, 220101, 45000);
    test(part1, 220100, 69206);
    test(part2, 220100, 197400);
}

fn part1(input: &str) -> u32 {
    sum_top(input, 1)
}

fn part2(input: &str) -> u32 {
    sum_top(input, 3)
}

fn sum_top(input: &str, n: usize) -> u32 {
    let mut xs = parse(input.as_ref())
        .into_iter()
        .map(|v| v.into_iter().sum::<u32>())
        .collect::<Vec<_>>();
    xs.sort_unstable();
    xs.reverse();
    xs.into_iter().take(n).sum()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    use nom::character::complete::*;
    use nom::multi::*;
    use nom::IResult;
    let parsed: IResult<&str, Vec<Vec<u32>>> =
        separated_list1(many1(line_ending), separated_list1(line_ending, u32))(input);
    parsed.expect("bad data").1
}
