use crate::util::parse_and_test;
use derive_more::From;
use itertools::Itertools as _;
use nom::Err;
use std::{collections::VecDeque, fmt::Display, ops::Range, str::FromStr};

#[test]
fn test1() {
    parse_and_test(part1, 230501, 35);
}

#[test]
fn test2() {
    parse_and_test(part1, 230500, 175622908);
}

#[test]
fn test3() {
    parse_and_test(part2, 230501, 46);
}

#[test]
fn test4() {
    parse_and_test(part2, 230500, 5200543);
}

#[derive(From, Clone, Debug)]
struct Almanac {
    starts: Vec<i64>,
    shiftses: Vec<Vec<Shift>>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;

        let header = || many_till(none_of(":"), tag(":")).and(multispace0);
        let header = || preceded(header(), success(()));
        let starts = preceded(header(), separated_list1(space1, i64));
        let shift = separated_list1(space1, i64);
        let shift = map_opt(shift, |v| v.into_iter().collect_tuple());
        let shift = shift.map(|(dst, src, len)| Shift::new(src, dst, len));
        let shifts = preceded(header(), separated_list1(multispace1, shift));
        let shiftses = separated_list1(multispace1, shifts);
        let almanac = tuple((starts, shiftses)).map(Almanac::from);
        let almanac = terminated(almanac, tuple((multispace0, eof)));

        almanac.anyhow(s)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, From)]
struct Interval {
    lb: i64,
    ub: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Shift {
    src: Interval,
    dst: Interval,
}

#[derive(Default, Debug)]
struct Overlap {
    prev: Option<Interval>,
    curr: Option<Interval>,
    next: Option<Interval>,
}

impl Interval {
    fn contains(&self, x: &i64) -> bool {
        (self.lb..self.ub).contains(x)
    }
}

impl Shift {
    fn new(src: i64, dst: i64, len: i64) -> Self {
        let src = (src, src + len).into();
        let dst = (dst, dst + len).into();
        Self { src, dst }
    }

    fn shift(&self, x: &mut i64) {
        *x -= self.src.lb;
        *x += self.dst.lb;
    }

    fn shift_interval(&self, Interval { lb, ub }: &mut Interval) {
        self.shift(lb);
        self.shift(ub);
    }
}

impl Overlap {
    fn compute(huge: &Interval, tiny: &Interval) -> Self {
        let this = Interval::from((
            huge.lb.clamp(tiny.lb, tiny.ub),
            huge.ub.clamp(tiny.lb, tiny.ub),
        ));
        Self {
            prev: (huge.lb < this.lb).then_some(Interval::from((huge.lb, this.lb))),
            next: (this.ub < huge.ub).then_some(Interval::from((this.ub, huge.ub))),
            curr: (this.lb < this.ub).then_some(Interval::from((this.lb, this.ub))),
        }
    }
}

fn part1(Almanac { starts, shiftses }: Almanac) -> i64 {
    let mut starts = starts;
    for shifts in shiftses.iter() {
        for start in starts.iter_mut() {
            if let Some(shift) = shifts.iter().find(|shift| shift.src.contains(start)) {
                shift.shift(start)
            }
        }
    }
    starts.into_iter().min().unwrap_or(0)
}

fn part2(Almanac { starts, shiftses }: Almanac) -> i64 {
    let mut old_intervals: VecDeque<Interval> = starts
        .into_iter()
        .chunks(2)
        .into_iter()
        .flat_map(|chunk| chunk.into_iter().collect_tuple())
        .map(|(lb, len)| (lb, lb + len).into())
        .collect();
    for shifts in shiftses {
        let mut new_intervals: VecDeque<Interval> = std::iter::empty().collect();
        'outer: while let Some(old_interval) = old_intervals.pop_front() {
            for &shift in shifts.iter() {
                let mut overlap = Overlap::compute(&old_interval, &shift.src);
                if let Some(ref mut the_match) = overlap.curr {
                    shift.shift_interval(the_match);
                    new_intervals.push_back(*the_match);
                    old_intervals.extend(overlap.prev.into_iter());
                    old_intervals.extend(overlap.next.into_iter());
                    continue 'outer;
                }
            }
            new_intervals.push_back(old_interval);
        }
        old_intervals = new_intervals;
    }
    old_intervals.into_iter().map(|i| i.lb).min().unwrap_or(0)
}
