#![allow(unstable_name_collisions)]

use itertools::Group;
use test_case::test_case;

use crate::util::*;

aoc_test!(part1, 231201, 21);
aoc_test!(part1, 231200, 7251);
aoc_test!(part2, 231201, 525152);

// SLOW TEST
// aoc_test!(part2, 231200, 2128386729962);

fn part1(records: Records) -> usize {
    records.count_matches()
}

fn part2(records: Records) -> usize {
    records.count_extended_matches()
}

struct Records(Vec<Record>);

#[derive(Debug, From)]
struct Record(Conditions, GroupSizes);

#[derive(Debug, From)]
struct Conditions(Vec<Condition>);

#[derive(Debug, From)]
struct GroupSizes(Vec<usize>);

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Copy, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Records {
    fn count_matches(&self) -> usize {
        self.0.iter().map(Record::n_matches).sum()
    }

    fn count_extended_matches(&self) -> usize {
        self.extend().count_matches()
    }

    fn extend(&self) -> Self {
        Self(self.0.iter().map(Record::extend).collect())
    }
}

impl Record {
    fn n_matches(&self) -> usize {
        let Record(conditions, group_sizes) = &self;
        Pats::build(group_sizes.as_slice()).apply(conditions.as_slice())
    }

    fn extend(&self) -> Self {
        let Record(conditions, group_sizes) = &self;
        Record(conditions.extend(), group_sizes.extend())
    }
}

impl GroupSizes {
    fn as_slice(&self) -> &[usize] {
        self.0.as_slice()
    }

    fn extend(&self) -> GroupSizes {
        Self(
            std::iter::repeat_with(|| self.0.clone())
                .take(5)
                .flatten()
                .collect(),
        )
    }
}
impl Conditions {
    fn as_slice(&self) -> &[Condition] {
        self.0.as_slice()
    }

    fn extend(&self) -> Conditions {
        Self(
            std::iter::repeat_with(|| self.0.clone())
                .take(5)
                .intersperse(vec![Condition::Unknown])
                .flatten()
                .collect(),
        )
    }
}

impl Condition {
    fn matches(&self, other: &Condition) -> bool {
        self == &Condition::Unknown || other == &Condition::Unknown || self == other
    }
}

#[derive(Debug, Clone)]
struct Pats(Vec<Pat>);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Pat {
    One(Condition),
    Any(Condition),
}

#[derive(Default)]
struct Matcher<'a> {
    memo: HashMap<(&'a [Condition], &'a [Pat]), usize>,
}

impl<'a> Matcher<'a> {
    fn n_matches(&mut self, ccc: &'a [Condition], ppp: &'a [Pat]) -> usize {
        let k = (ccc, ppp);

        if let Some(n) = self.memo.get(&k) {
            *n
        } else {
            let n = self.n_matches_compute(ccc, ppp);
            self.memo.insert(k, n);
            n
        }
    }

    fn n_matches_compute(&mut self, ccc: &'a [Condition], ppp: &'a [Pat]) -> usize {
        use {Condition::*, Pat::*};
        match (ccc.split_first(), ppp.split_first()) {
            (None, None) => 1,
            (None, Some((Any(_), pp))) => self.n_matches(ccc, pp),
            (Some((c, cc)), Some((One(p), pp))) if c.matches(p) => self.n_matches(cc, pp),
            (Some((c, cc)), Some((Any(p), pp))) if c.matches(p) => {
                self.n_matches(cc, ppp) + self.n_matches(ccc, pp)
            }
            (Some((_, _)), Some((Any(_), pp))) => self.n_matches(ccc, pp),
            _ => 0,
        }
    }
}

impl Pats {
    fn build(group_sizes: &[usize]) -> Pats {
        use Condition::*;
        use Pat::*;

        let pattern = itertools::chain!(
            vec![Any(Operational)],
            group_sizes
                .iter()
                .map(|n| vec![One(Damaged); *n])
                .intersperse(vec![One(Operational), Any(Operational)])
                .flatten(),
            vec![Any(Operational)]
        );
        Pats(pattern.collect())
    }

    fn apply(&self, conditions: &[Condition]) -> usize {
        Matcher::default().n_matches(conditions, self.as_slice())
    }

    fn as_slice(&self) -> &[Pat] {
        self.0.as_slice()
    }
}

#[test_case("???.### 1,1,3", 1)]
#[test_case(".??..??...?##. 1,1,3", 4)]
#[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
#[test_case("????.#...#... 4,1,1", 1)]
#[test_case("????.######..#####. 1,6,5", 4)]
#[test_case("?###???????? 3,2,1", 10)]
fn test(record: &str, expected: usize) {
    let record = record.parse::<Record>().unwrap();
    assert_eq!(record.n_matches(), expected);
}

impl FromStr for Records {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let records = s
            .lines()
            .map(Record::from_str)
            .collect::<anyhow::Result<_>>()?;
        Ok(Records(records))
    }
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let condition = fail
            .into_str_parser()
            .or(tag("?").map(|_| Condition::Unknown))
            .or(tag(".").map(|_| Condition::Operational))
            .or(tag("#").map(|_| Condition::Damaged));
        let usize = u32.map(|n| n as usize);
        let group_sizes = separated_list1(tag(","), usize)
            .into_str_parser()
            .map(GroupSizes::from);
        let conditions = many1(condition).map(Conditions::from);
        let record = separated_pair(conditions, space1, group_sizes).map(Record::from);
        record.anyhow(s)
    }
}

impl Display for Pats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for frob in &self.0 {
            write!(f, "{frob}")?;
        }
        Ok(())
    }
}

impl Display for Pat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pat::One(c) => write!(f, "{c}"),
            Pat::Any(c) => write!(f, "{c}*"),
        }
    }
}

impl Display for Conditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for condition in &self.0 {
            write!(f, "{condition}")?;
        }
        Ok(())
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Operational => write!(f, "."),
            Condition::Damaged => write!(f, "#"),
            Condition::Unknown => write!(f, "?"),
        }
    }
}
