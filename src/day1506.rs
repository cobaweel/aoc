use ndarray::{s, Array2, ArrayViewMut2};

use crate::util::*;

aoc_test!(part1, 150600, 400410);
aoc_test!(part2, 150600, 15343601);

#[derive(Debug, From)]
struct Instructions(Vec<Instruction>);

#[derive(Debug, From, Clone, Copy)]
struct Instruction(Operation, Light, Light);

#[derive(Debug, Clone, Copy)]
enum Operation {
    Toggle,
    On,
    Off,
}

#[derive(Debug, From, Clone, Copy)]
struct Light(usize, usize);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let operation = alt((
            tag("toggle").map(|_| Operation::Toggle),
            tag("turn on").map(|_| Operation::On),
            tag("turn off").map(|_| Operation::Off),
        ));
        let usize = || u32.map(|u| u as usize);
        let light = || separated_pair(usize(), tag(","), usize()).map(Light::from);
        let instruction = tuple((
            terminated(operation, space1),
            terminated(light(), tag(" through ")),
            light(),
        ))
        .map(Instruction::from);
        let instructions = separated_list1(multispace1, instruction).map(Instructions::from);
        instructions.anyhow(s)
    }
}

impl Instructions {
    fn execute<T>(&self, array: &mut Array2<T>, f: impl Fn(Operation, &mut T)) {
        for instruction in self.0.iter() {
            instruction.execute(array, &f);
        }
    }
}

impl Instruction {
    fn execute<T>(&self, array: &mut Array2<T>, f: impl Fn(Operation, &mut T)) {
        let &Instruction(operation, Light(x_lo, y_lo), Light(x_hi, y_hi)) = self;
        for t in array.slice_mut(s![x_lo..=x_hi, y_lo..=y_hi]).iter_mut() {
            f(operation, t);
        }
    }
}

fn part1(instructions: Instructions) -> usize {
    let mut bulbs = Array2::from_elem((1000, 1000), false);
    instructions.execute(&mut bulbs, |operation, bulb| {
        *bulb = match (&operation, *bulb) {
            (Operation::Toggle, true) => false,
            (Operation::Toggle, false) => true,
            (Operation::On, _) => true,
            (Operation::Off, _) => false,
        };
    });
    bulbs.into_iter().filter(|&b| b).count()
}

fn part2(instructions: Instructions) -> u64 {
    let mut bulbs = Array2::from_elem((1000, 1000), 0u64);
    instructions.execute(&mut bulbs, |operation, bulb| {
        let change : i64 = match &operation {
            Operation::Toggle => 2,
            Operation::On => 1,
            Operation::Off => -1,
        };
        *bulb = bulb.saturating_add_signed(change);
    });
    bulbs.into_iter().sum()
}
