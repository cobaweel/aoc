use crate::util::*;

aoc_test!(part1, 150100, 232);
aoc_test!(part2, 150100, 1783);

fn part1(moves: String) -> i32 {
    floors(moves.as_str()).last().unwrap()
}

fn part2(moves: String) -> usize {
    let n_floors = floors(moves.as_str())
        .take_while(|&floor| floor >= 0)
        .count();
    n_floors + 1
}

fn floors(moves: &str) -> impl Iterator<Item = i32> + '_ {
    moves.chars().scan(0, |floor, c| {
        match c {
            '(' => *floor += 1,
            ')' => *floor -= 1,
            _ => {}
        };
        Some(*floor)
    })
}
