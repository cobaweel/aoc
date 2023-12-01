use itertools::Itertools as _;
use nom::character::streaming::anychar;
use std::str::FromStr;

#[test]
fn test1() {
    crate::util::parse_and_test(part1, 221101, 10605);
}

#[test]
fn test2() {
    crate::util::parse_and_test(part1, 221100, 111210);
}

#[test]
fn test3() {
    crate::util::parse_and_test(part2, 221101, 2713310158);
}

#[test]
fn test4() {
    crate::util::parse_and_test(part2, 221100, 15447387620);
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    decision: Decision,
}

impl From<(Vec<u64>, Operation, Decision)> for Monkey {
    fn from((items, operation, decision): (Vec<u64>, Operation, Decision)) -> Self {
        Monkey {
            items,
            operation,
            decision,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sqr,
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Operation::Add(i) => x + i,
            Operation::Mul(i) => x * i,
            Operation::Sqr => x * x,
        }
    }
}

#[derive(Debug)]
struct Decision {
    divisor: u64,
    if_yes: u64,
    if_no: u64,
}

impl From<(u64, u64, u64)> for Decision {
    fn from((divisor, if_yes, if_no): (u64, u64, u64)) -> Self {
        Decision {
            divisor,
            if_yes,
            if_no,
        }
    }
}

impl Decision {
    fn apply(&self, x: u64) -> u64 {
        if x % self.divisor == 0 {
            self.if_yes
        } else {
            self.if_no
        }
    }
}

impl FromStr for Monkeys {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;
        let label = || tuple((many0(none_of(":")), tag(":"), space0));
        let items = preceded(label(), separated_list1(tag(", "), u64));
        let sqr = tag("* old").map(|_| Operation::Sqr);
        let mul = preceded(tag("* "), u64).map(Operation::Mul);
        let add = preceded(tag("+ "), u64).map(Operation::Add);
        let operation = preceded(many0(none_of("*+")), alt((sqr, mul, add)));
        let bla = || preceded(many0(none_of("0123456789")), u64);
        let decision = tuple((bla(), bla(), bla())).map(Decision::from);
        let monkey = preceded(label(), tuple((items, operation, decision)));
        let monkey = monkey.map(Monkey::from);
        let monkeys = terminated(many1(monkey), tuple((multispace0, eof)));
        let monkeys = monkeys.map(Monkeys::new).parse(s).munch()?;
        Ok(monkeys)
    }
}

fn part1(Monkeys { mut monkeys }: Monkeys) -> u64 {
    let mut activity = monkeys.iter().map(|_| 0_u64).collect_vec();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            for item in items {
                activity[i] += 1;
                let item = monkeys[i].operation.apply(item) / 3;
                let target = monkeys[i].decision.apply(item) as usize;
                monkeys[target].items.push(item);
            }
        }
    }
    activity.into_iter().sorted().rev().take(2).product::<u64>()
}

fn part2(Monkeys { mut monkeys }: Monkeys) -> u64 {
    let mut activity = monkeys.iter().map(|_| 0_u64).collect_vec();
    let field: u64 = monkeys
        .iter()
        .map(|monkey| monkey.decision.divisor)
        .product();
    println!("{field}");
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            for item in items {
                activity[i] += 1;
                let item = monkeys[i].operation.apply(item);
                let item = item % field;
                let target = monkeys[i].decision.apply(item) as usize;
                monkeys[target].items.push(item);
            }
        }
    }
    activity.into_iter().sorted().rev().take(2).product::<u64>()
}
