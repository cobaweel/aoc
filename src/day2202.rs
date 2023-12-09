#[test]
fn test() {
    use crate::util::test;
    test(part1, 220201, 15);
    test(part1, 220200, 10994);
    test(part2, 220201, 12);
    test(part2, 220200, 12526);
}

fn part1(input: &str) -> u32 {
    let plays = input.lines().map(parse_line1);
    let scores = plays.map(|(p, q)| score1(&p, &q));
    let total: u32 = scores.sum();
    total
}

fn part2(input: &str) -> u32 {
    let plays = input.lines().map(parse_line2);
    let scores = plays.map(|(p, q)| score2(&p, &q));
    let total: u32 = scores.sum();
    total
}

#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn rps(v: &str) -> RPS {
    use RPS::*;
    match v {
        "X" | "A" => Rock,
        "Y" | "B" => Paper,
        "Z" | "C" => Scissors,
        _ => panic!(),
    }
}

fn battle(p: &RPS, q: &RPS) -> u32 {
    use RPS::*;
    match (p, q) {
        (Rock, Rock) => 3,
        (Paper, Paper) => 3,
        (Scissors, Scissors) => 3,
        (Rock, Scissors) => 6,
        (Paper, Rock) => 6,
        (Scissors, Paper) => 6,
        _ => 0,
    }
}

fn base(p: &RPS) -> u32 {
    use RPS::*;
    match p {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn parse_line1(line: &str) -> (RPS, RPS) {
    let (p, q) = line.split_once(' ').expect("syntax");
    (rps(p), rps(q))
}

fn score1(p: &RPS, q: &RPS) -> u32 {
    base(q) + battle(q, p)
}

fn parse_line2(line: &str) -> (RPS, Outcome) {
    use Outcome::*;
    let (p, q) = line.split_once(' ').expect("syntax");
    (
        rps(p),
        match q {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!(),
        },
    )
}

fn score2(p: &RPS, outcome: &Outcome) -> u32 {
    use Outcome::*;
    use RPS::*;
    let q = match (p, outcome) {
        (Rock, Draw) => Rock,
        (Paper, Draw) => Paper,
        (Scissors, Draw) => Scissors,
        (Rock, Win) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Win) => Rock,
        (Rock, Lose) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Lose) => Paper,
    };
    base(&q) + battle(&q, p)
}
