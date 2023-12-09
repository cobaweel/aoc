use itertools::Itertools;

#[test]
fn test() {
    use crate::util::test;
    test(part1, 220501, "CMZ".to_string());
    test(part1, 220500, "PTWLTDSJV".to_string());
    test(part2, 220501, "MCD".to_string());
    test(part2, 220500, "WZMFVGGZP".to_string());
}

struct Move {
    n: usize,
    src: usize,
    dst: usize,
}

fn part1(input: &str) -> String {
    let mut piles = parse_piles(input);
    for Move { n, src, dst } in parse_moves(input) {
        for _ in 0..n {
            if let Some(c) = piles[src - 1].pop() {
                piles[dst - 1].push(c);
            }
        }
    }
    get_tops(piles)
}

fn part2(input: &str) -> String {
    let mut piles = parse_piles(input);
    for Move { n, src, dst } in parse_moves(input) {
        let split_idx = piles[src - 1].len() - n;
        let mut stack = piles[src - 1].split_off(split_idx);
        piles[dst - 1].append(&mut stack);
    }
    get_tops(piles)
}

fn get_tops(piles: Vec<Vec<char>>) -> String {
    piles.into_iter().flat_map(|mut pile| pile.pop()).join("")
}

fn parse_moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .flat_map(|line| {
            line.split_whitespace()
                .dropping(1)
                .step_by(2)
                .flat_map(|s| s.parse::<usize>())
                .collect_tuple::<(usize, usize, usize)>()
        })
        .map(|(n, src, dst)| Move { n, src, dst })
}

fn parse_piles(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| line.trim_start().starts_with("["))
        .rev()
        .flat_map(|line| line.chars().dropping(1).step_by(4).enumerate())
        .filter(|(_, c)| c != &' ')
        .sorted_by_key(|(i, _)| *i)
        .into_iter()
        .group_by(|(i, _)| *i)
        .into_iter()
        .map(|(_, group)| group.map(|(_, c)| c).collect_vec())
        .collect_vec()
}
