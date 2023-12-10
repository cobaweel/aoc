use crate::util::*;

#[test]
fn test1() {
    aoc_parse_and_test(part1, 230601, 288);
}

#[test]
fn test2() {
    aoc_parse_and_test(part1, 230600, 1710720);
}

#[test]
fn test3() {
    aoc_parse_and_test(part2, 230601, 71503);
}

#[test]
fn test4() {
    aoc_parse_and_test(part2, 230600, 35349468);
}

struct Records {
    records: Vec<Record>,
}

impl Records {
    fn new(records: Vec<Record>) -> Self {
        Self { records }
    }

    fn iter(&self) -> impl Iterator<Item = &Record> {
        self.records.iter()
    }
}

impl FromStr for Records {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;
        let prefix = || tuple((many1(none_of(":")), tag(":"), space1));
        let numbers = || preceded(prefix(), separated_list1(space1, i64));
        let records = || {
            separated_pair(numbers(), multispace1, numbers()).map(|(times, distances)| {
                let records = itertools::izip!(times, distances)
                    .map(|(time, distance)| Record { time, distance })
                    .collect_vec();
                Records { records }
            })
        };
        records().anyhow(s)
    }
}

#[derive(From)]
struct Record {
    time: i64,
    distance: i64,
}

fn part1(records: Records) -> i64 {
    records.records.into_iter().map(n_ways_to_win).product()
}

fn n_ways_to_win(Record { time, distance }: Record) -> i64 {
    (0..=time).filter(|t| t * (time - t) > distance).count() as i64
}

fn part2(records: Records) -> i64 {
    let records = records
        .iter()
        .map(|Record { time, distance }| (time, distance));
    let (times, distances) = records.multiunzip::<(Vec<i64>, Vec<i64>)>();
    let time = times.iter().join("").parse::<i64>().unwrap_or(0);
    let distance = distances.iter().join("").parse::<i64>().unwrap_or(0);
    n_ways_to_win(Record { time, distance })
}
