use crate::util::*;
use std::num::ParseIntError;
use std::str::FromStr;

aoc_test!(part1, 150200, 1606483);
aoc_test!(part2, 150200, 3842356);

#[derive(From)]
struct Gifts {
    gifts: Vec<Gift>,
}

struct Gift(u32, u32, u32);

impl FromStr for Gifts {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let gift = separated_list1(tag("x"), u32);
        let gift = map_opt(gift, |dims| dims.into_iter().collect_tuple());
        let gift = gift.map(|(l, w, h)| Gift(l, w, h));
        let gifts = separated_list1(multispace1, gift);
        let gifts = gifts.map(Gifts::from);
        gifts.anyhow(s)
    }
}

fn part1(gifts: Gifts) -> u32 {
    gifts.sum(Gift::wrap)
}

fn part2(gifts: Gifts) -> u32 {
    gifts.sum(Gift::ribbon)
}

impl Gifts {
    fn sum(self, f: impl Fn(Gift) -> u32) -> u32 {
        self.gifts.into_iter().map(f).sum::<u32>()
    }
}

impl Gift {
    fn wrap(self) -> u32 {
        let Gift(l, w, h) = self;
        let areas = [l * w, w * h, l * h];
        let x = 2 * areas.into_iter().sum::<u32>();
        let y = areas.into_iter().min().unwrap_or(0);
        x + y
    }

    fn ribbon(self) -> u32 {
        let Gift(l, w, h) = self;
        let perimeter = [l, w, h].into_iter().sorted().take(2).sum::<u32>() * 2;
        let volume = l * w * h;
        perimeter + volume
    }
}
