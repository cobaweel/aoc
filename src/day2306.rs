use crate::util::*;

aoc_test!(part1, 230601, 288);
aoc_test!(part1, 230600, 1710720);
aoc_test!(part2, 230601, 71503);
aoc_test!(part2, 230600, 35349468);

struct Records {
    records: Vec<Record>,
}

#[derive(From)]
struct Record {
    time: u64,
    distance: u64,
}

impl FromStr for Records {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::aoc_nom::*;
        let prefix = || tuple((many1(none_of(":")), tag(":"), space1));
        let numbers = || preceded(prefix(), separated_list1(space1, u64));
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

fn part1(records: Records) -> u64 {
    records.ways_to_win().product()
}

fn part2(records: Records) -> u64 {
    records.combine().ways_to_win()
}

impl Record {
    fn ways_to_win(&self) -> u64 {
        let (t0, t1) = self.break_even_int();
        (t1 + 1).abs_diff(t0)
    }

    fn is_winning(&self, t: u64) -> bool {
        t * (self.time - t) > self.distance
    }

    fn break_even_int(&self) -> (u64, u64) {
        let (t0, t1) = self.break_even_float();
        let mut t0 = t0.ceil() as u64 + 1;
        let mut t1 = t1.floor() as u64 - 1;
        while t0 > 0 && self.is_winning(t0 - 1) {
            t0 -= 1;
        }
        while self.is_winning(t1 + 1) {
            t1 += 1;
        }
        (t0, t1)
    }

    fn break_even_float(&self) -> (f64, f64) {
        let &Record { time, distance } = self;
        let time = time as f64;
        let distance = distance as f64;
        let t0 = (time - (time * time - 4. * distance).sqrt()) / 2.;
        let t1 = (time + (time * time - 4. * distance).sqrt()) / 2.;
        (t0.min(t1), t0.max(t1))
    }
}

impl Records {
    fn ways_to_win(&self) -> impl Iterator<Item = u64> + '_ {
        self.records.iter().map(Record::ways_to_win)
    }

    fn combine(self) -> Record {
        let records = self
            .records
            .iter()
            .map(|Record { time, distance }| (time, distance));
        let (times, distances) = records.multiunzip::<(Vec<u64>, Vec<u64>)>();
        let time = times.iter().join("").parse::<u64>().unwrap_or(0);
        let distance = distances.iter().join("").parse::<u64>().unwrap_or(0);
        Record { time, distance }
    }
}
