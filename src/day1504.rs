use crate::util::*;

// These tests are commented out because AdventCoin mining makes my
// computer into a space heater, and takes too long.

// aoc_test!(part1, 150400, 346386);
// aoc_test!(part2, 150400, 9958218);

fn part1(input: String) -> usize {
    mine_advent_coint("00000", input.as_str())
}

fn part2(input: String) -> usize {
    mine_advent_coint("000000", input.as_str())
}

fn mine_advent_coint(prefix: &str, input: &str) -> usize {
    use md5::Digest as _;
    let mut i: usize = 0;
    loop {
        let hashed = format!("{}{}", input, i);
        let hash = format!("{:x}", md5::compute(hashed));
        if hash.starts_with(prefix) {
            return i;
        }
        i += 1;
    }
}
