use itertools::Itertools as _;
use crate::util::parse_and_test;
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

#[derive(Clone, Debug)]
struct Almanac {
    starts: Vec<i64>,
    shiftses: Vec<Vec<Shift>>,
}

impl From<(Vec<i64>, Vec<Vec<Shift>>)> for Almanac {
    fn from((starts, shiftses): (Vec<i64>, Vec<Vec<Shift>>)) -> Self {
        Almanac { starts, shiftses }
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;

        let shift = tuple((i64, space1, i64, space1, i64));
        let shift = shift.map(|(src, _, dst, _, len)| Shift::new(src, dst, len));
        let header = || many_till(none_of(":"), tag(":")).and(space0);
        let shifts = preceded(header(), separated_list1(multispace1, shift));
        let starts = preceded(header(), separated_list1(space1, i64));
        let shiftses = many1(shifts);
        let almanac = || tuple((starts, shiftses)).map(Almanac::from);
        let almanac = almanac().parse(s).munch()?;
        Ok(almanac)
    }
}

#[derive(Debug, Copy, Clone)]
struct Interval {
    lb: i64,
    ub: i64,
}

#[derive(Debug, Copy, Clone)]
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
    fn new(lb: i64, ub: i64) -> Self {
        Self { lb, ub }
    }

    fn contains(&self, x: &i64) -> bool {
        (self.lb..self.ub).contains(x)
    }
}

impl Shift {
    fn new(src: i64, dst: i64, len: i64) -> Self {
        let src = Interval::new(src, src + len);
        let dst = Interval::new(dst, dst + len);
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
    fn overlap(huge: &Interval, tiny: &Interval) -> Self {
        let this = Interval::new(
            huge.lb.clamp(tiny.lb, tiny.ub),
            huge.ub.clamp(tiny.lb, tiny.ub),
        );
        Self {
            prev: (huge.lb < this.lb).then_some(Interval::new(huge.lb, this.lb)),
            curr: (this.lb < this.ub).then_some(Interval::new(this.lb, this.ub)),
            next: (this.ub < huge.ub).then_some(Interval::new(this.ub, huge.ub)),
        }
    }
}

fn part1(
    Almanac {
        starts,
        shiftses: mappingses,
    }: Almanac,
) -> i64 {
    let mut starts = starts;
    for mappings in mappingses.iter() {
        for start in starts.iter_mut() {
            mappings
                .iter()
                .find(|m| m.src.contains(start))
                .map(|m| m.shift(start));
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
        .map(|(lb, len)| Interval::new(lb, lb + len))
        .collect();
    for shifts in shiftses {
        let mut new_intervals: VecDeque<Interval> = std::iter::empty().collect();
        'outer: while let Some(old_interval) = old_intervals.pop_front() {
            for &shift in shifts.iter() {
                let mut overlap = Overlap::overlap(&old_interval, &shift.src);
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
