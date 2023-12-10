use crate::util::*;

aoc_test!(part1, 230901, 114);
aoc_test!(part1, 230900, 1884768153);
aoc_test!(part2, 230901, 2);
aoc_test!(part2, 230900, 1031);

#[derive(From)]
struct Oasis {
    histories: Vec<History>,
}

#[derive(From)]
struct History {
    values: Vec<i64>,
}

impl FromStr for Oasis {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let history = separated_list1(space1, i64).map(History::from);
        let oasis = separated_list1(multispace1, history).map(Oasis::from);
        oasis.anyhow(s)
    }
}

fn part1(oasis: Oasis) -> i64 {
    oasis.histories.into_iter().map(forecast).sum()
}

fn part2(oasis: Oasis) -> i64 {
    oasis.histories.into_iter().map(hindcast).sum()
}

fn differentiate(values: &Vec<i64>) -> Vec<i64> {
    values
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn is_not_zeroes(values: &Vec<i64>) -> bool {
    !values.iter().all(|&v| v == 0)
}

fn forecast(history: History) -> i64 {
    foldcast(history, |d, derivative| d + derivative.last().unwrap())
}

fn hindcast(history: History) -> i64 {
    foldcast(history, |d, derivative| derivative.first().unwrap() - d)
}

fn foldcast(history: History, f: impl Fn(i64, Vec<i64>) -> i64) -> i64 {
    itertools::iterate(history.values, differentiate)
        .take_while_inclusive(is_not_zeroes)
        .collect_vec()
        .into_iter()
        .rev()
        .fold(0, f)
}

