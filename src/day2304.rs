use crate::util::*;

aoc_test!(part1, 230401, 13);
aoc_test!(part1, 230400, 19135);
aoc_test!(part2, 230401, 30);
aoc_test!(part2, 230400, 5704953);

struct Cards(Vec<Card>);

impl FromStr for Cards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.lines().map(|line| line.parse()).try_collect()?;
        Ok(Cards(cards))
    }
}

struct Card {
    mine: Vec<usize>,
    wins: Vec<usize>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mine, wins) = s
            .split(|c| ":|".contains(c))
            .collect_tuple()
            .ok_or(anyhow::anyhow!("bad"))?;
        let mine: Vec<usize> = mine
            .split_ascii_whitespace()
            .flat_map(|n| n.parse())
            .collect_vec();
        let wins: Vec<usize> = wins
            .split_ascii_whitespace()
            .flat_map(|n| n.parse())
            .collect_vec();
        Ok(Card { mine, wins })
    }
}

impl Cards {
    fn matches(self) -> Vec<usize> {
        self.0.into_iter().map(Card::matches).collect_vec()
    }
}

impl Card {
    fn matches(self) -> usize {
        let mine = self.mine.into_iter().collect::<HashSet<_>>();
        let wins = self.wins.into_iter().collect::<HashSet<_>>();
        mine.intersection(&wins).count()
    }
}

fn part1(cards: Cards) -> usize {
    let scores = cards.matches().into_iter().map(|n| {
        if n > 0 {
            2_usize.pow((n - 1) as u32)
        } else {
            0
        }
    });
    scores.sum()
}

fn part2(cards: Cards) -> usize {
    let scores: Vec<usize> = cards.matches();
    fn add_up(scores: &[usize], i: usize) -> usize {
        (i + 1..=i + scores[i])
            .map(|j| add_up(scores, j))
            .sum::<usize>()
            + 1
    }
    (0..scores.len())
        .map(|i| add_up(scores.as_slice(), i))
        .sum()
}
