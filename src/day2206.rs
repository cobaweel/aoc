use crate::util::*;

aoc_test!(part1, 220601, 7);
aoc_test!(part1, 220600, 1361);
aoc_test!(part2, 220601, 19);
aoc_test!(part2, 220600, 3263);

fn part1(input: &str) -> usize {
    find_marker_of_size(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker_of_size(input, 14)
}

fn find_marker_of_size(input: &str, n: usize) -> usize {
    use std::collections::HashSet;
    input
        .chars()
        .collect_vec()
        .windows(n)
        .find_position(|window| window.iter().collect::<HashSet<_>>().len() == n)
        .expect("not found")
        .0
        + n
}
