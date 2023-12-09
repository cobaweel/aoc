use crate::util::parse_and_test;
use itertools::Itertools as _;
use ndarray::{Array1, Array2};

#[test]
fn test1() {
    parse_and_test(part1, 221001, 13140);
}

#[test]
fn test2() {
    parse_and_test(part1, 221000, 14780);
}

#[test]
fn test3() {
    let screen = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    parse_and_test(part2, 221001, screen.to_string());
}

#[test]
fn test4() {
    let screen = "\
####.#....###..#....####..##..####.#....
#....#....#..#.#.......#.#..#....#.#....
###..#....#..#.#......#..#......#..#....
#....#....###..#.....#...#.##..#...#....
#....#....#....#....#....#..#.#....#....
####.####.#....####.####..###.####.####.";
    parse_and_test(part2, 221000, screen.to_string());
}

#[derive(Debug)]
struct Program {
    ops: Vec<Op>,
}

impl Program {
    fn new(ops: Vec<Op>) -> Self {
        Self { ops }
    }

    fn as_micro_instructions(self) -> impl Iterator<Item = i32> {
        self.ops.into_iter().flat_map(|op| match op {
            Op::Noop => vec![0],
            Op::Addx(i) => vec![0, i],
        })
    }
}

impl std::str::FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::parse_with_nom::*;
        let noop = tag("noop").map(|_| Op::Noop);
        let addx = preceded(tuple((tag("addx"), space1)), i32).map(Op::Addx);
        let op = alt((noop, addx));
        let mut program = separated_list0(multispace1, op).map(Program::new);
        program.parse(s).munch()
    }
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx(i32),
}

fn part1(program: Program) -> i32 {
    run(program)
        .zip(1..)
        .skip(19)
        .step_by(40)
        .map(|(cycle, strength)| cycle * strength)
        .sum()
}

fn run(program: Program) -> impl Iterator<Item = i32> {
    let mut x = 1;
    program
        .as_micro_instructions()
        .map(move |i| {
            x += i;
            x - i
        })
        .into_iter()
}

fn draw_scan_line(line: impl Iterator<Item = i32>) -> String {
    line.zip(0..)
        .map(|(sprite_pos, i)| {
            let i = i as i32;
            if (sprite_pos - 1..=sprite_pos + 1).contains(&i) {
                '#'
            } else {
                '.'
            }
        })
        .join("")
}

fn part2(program: Program) -> String {
    let screen = run(program)
        .chunks(40)
        .into_iter()
        .map(draw_scan_line)
        .join("\n");
    println!("{}", screen);
    screen
}
