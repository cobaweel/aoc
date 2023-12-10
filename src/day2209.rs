use crate::util::*;

aoc_test!(part1, 220901, 13);
aoc_test!(part1, 220900, 6212);
// aoc_test!(part2, 220901, 1);
// aoc_test!(part2, 220900, -1);

struct Move(Direction, u32);

#[derive(Debug)]
enum Direction {
    R,
    L,
    U,
    D,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use anyhow::anyhow;
        s.split_ascii_whitespace()
            .collect_tuple()
            .map(|(dir, dist)| {
                let dist = dist.parse().unwrap_or(0);
                match dir {
                    "R" => Move(Direction::R, dist),
                    "L" => Move(Direction::L, dist),
                    "U" => Move(Direction::U, dist),
                    _ => Move(Direction::D, dist),
                }
            })
            .ok_or(anyhow!("oops"))
    }
}

fn walk(x: &mut (i32, i32), dx: (i32, i32)) {
    x.0 += dx.0;
    x.1 += dx.1;
}

fn part1(input: &str) -> usize {
    let mut tail_positions = HashSet::new();
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    for Move(direction, dist) in parse(input) {
        for _ in 0..dist {
            lead(&mut head_position, &direction);
            follow(&mut tail_position, &head_position);
            tail_positions.insert(tail_position);
        }
    }
    tail_positions.len()
}

fn follow(follower: &mut (i32, i32), leader: &(i32, i32)) {
    let dx = leader.0 - follower.0;
    let dy = leader.1 - follower.1;
    let d_tail = match (dx.abs(), dy.abs()) {
        (0, 2) | (2, 0) | (1, 2) | (2, 1) => (dx.signum(), dy.signum()),
        _ => (0, 0),
    };
    walk(follower, d_tail);
}

fn lead(point: &mut (i32, i32), direction: &Direction) {
    use Direction::*;
    let d_head = match *direction {
        R => (1, 0),
        L => (-1, 0),
        U => (0, 1),
        D => (0, -1),
    };
    walk(point, d_head);
}

fn part2(_input: &str) -> usize {
    todo!()
}

fn parse(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().flat_map(|line| line.parse::<Move>())
}
