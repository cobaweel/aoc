use crate::util::*;

aoc_test!(part1, 230601, 288);
aoc_test!(part1, 230600, 1710720);
aoc_test!(part2, 230601, 71503);
aoc_test!(part2, 230600, 35349468);

struct Races {
    races: Vec<Race>,
}

#[derive(From)]
struct Race {
    time: u64,
    distance: u64,
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::aoc_nom::*;
        let prefix = || tuple((many1(none_of(":")), tag(":"), space1));
        let numbers = || preceded(prefix(), separated_list1(space1, u64));
        let races = || {
            separated_pair(numbers(), multispace1, numbers()).map(|(times, distances)| {
                let races = itertools::izip!(times, distances)
                    .map(|(time, distance)| Race { time, distance })
                    .collect_vec();
                Races { races }
            })
        };
        races().anyhow(s)
    }
}

fn part1(races: Races) -> u64 {
    races.ways_to_win().product()
}

fn part2(races: Races) -> u64 {
    races.combine().ways_to_win()
}

impl Race {
    /// How many distinct durations of button pushing will win us this race
    fn ways_to_win(&self) -> u64 {
        // To find out how many ways there are to win the race, we solve the
        // quadratic equation to get a rough idea, then round the answer to
        // integers very conservatively and manually check around the edges
        // exactly which integers do and don't qualify. This avoids edge cases
        // with floating point equality.
        let (t0, t1) = quadratic_roots(1., -(self.time as f64), self.distance as f64);
        let mut t0 = t0.ceil() as u64 + 1;
        let mut t1 = t1.floor() as u64 - 1;
        while t0 > 0 && self.is_winning(t0 - 1) {
            t0 -= 1;
        }
        while self.is_winning(t1 + 1) {
            t1 += 1;
        }
        (t1 + 1).abs_diff(t0)
    }

    /// See if holding the button for `t` seconds will win us this race
    fn is_winning(&self, t: u64) -> bool {
        t * (self.time - t) > self.distance
    }
}

impl Races {
    /// How many distinct durations of button pushing are there to win each of
    /// the races
    fn ways_to_win(&self) -> impl Iterator<Item = u64> + '_ {
        self.races.iter().map(Race::ways_to_win)
    }

    /// Combine all the records into one, as per part 2 of the problem
    fn combine(self) -> Race {
        let records = self
            .races
            .iter()
            .map(|Race { time, distance }| (time, distance));
        let (times, distances) = records.multiunzip::<(Vec<u64>, Vec<u64>)>();
        let time = times.iter().join("").parse::<u64>().unwrap_or(0);
        let distance = distances.iter().join("").parse::<u64>().unwrap_or(0);
        Race { time, distance }
    }
}

/// The standard high school algebra formula for the roots of a quadratic
/// equation. Roots come out in order.
fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = |k| (-b + k * (b * b - 4. * a * c).sqrt()) / (2. * a);
    let (p, q) = (root(-1.), root(1.));
    (p.min(q), p.max(q))
}
