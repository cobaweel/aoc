use crate::util::*;

#[test]
fn test1() {
    aoc_test(part1, 230201, 8);
    aoc_test(part1, 230200, 2776);
    aoc_test(part2, 230201, 2286);
    aoc_test(part2, 230200, 68638);
}

fn part1(input: &str) -> u32 {
    let limit_counter = Counter::new(12, 13, 14);
    input
        .lines()
        .map(parse)
        .zip(1..)
        .map(|(counters, id)| {
            if counters
                .fold(Counter::default(), |max_counter, counter| {
                    max_counter.update_max(&counter)
                })
                .fits_inside(&limit_counter)
            {
                id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse)
        .map(|counters| {
            counters
                .fold(Counter::default(), |max_counter, counter| {
                    max_counter.update_max(&counter)
                })
                .power()
        })
        .sum()
}

#[derive(Default, Debug)]
struct Counter {
    red: u32,
    green: u32,
    blue: u32,
}

impl Counter {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn add(&mut self, count: u32, color: &str) {
        match color {
            "red" => self.red += count,
            "green" => self.green += count,
            _ => self.blue += count,
        }
    }

    fn update_max(mut self, other: &Self) -> Self {
        use std::cmp::max;
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);
        self.blue = max(self.blue, other.blue);
        self
    }

    fn fits_inside(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse(line: &str) -> impl Iterator<Item = Counter> + '_ {
    line.split(|c| ":;".contains(c)).skip(1).map(|sample_str| {
        let mut sample = Counter::default();
        sample_str.split(',').for_each(|count_and_color| {
            count_and_color
                .split_ascii_whitespace()
                .collect_tuple()
                .into_iter()
                .for_each(|(count, color)| {
                    let count = count.parse().unwrap_or(0);
                    sample.add(count, color);
                });
        });
        sample
    })
}
