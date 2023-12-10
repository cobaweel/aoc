use crate::util::*;
use ndarray::{Array1, Array2};

const RESULT1: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

const RESULT2: &str = "\
####.#....###..#....####..##..####.#....
#....#....#..#.#.......#.#..#....#.#....
###..#....#..#.#......#..#......#..#....
#....#....###..#.....#...#.##..#...#....
#....#....#....#....#....#..#.#....#....
####.####.#....####.####..###.####.####.";

aoc_test!(part1, 221001, 13140);
aoc_test!(part1, 221000, 14780);
aoc_test!(part2, 221001, RESULT1.to_string());
aoc_test!(part2, 221000, RESULT2.to_string());

#[derive(Debug, From)]
struct Program {
    ops: Vec<Op>,
}

impl Program {
    fn into_micro_instructions(self) -> impl Iterator<Item = i32> {
        self.ops.into_iter().flat_map(|op| match op {
            Op::Noop => vec![0],
            Op::Addx(i) => vec![0, i],
        })
    }
}

impl std::str::FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::util::aoc_nom::*;
        let noop = tag("noop").map(|_| Op::Noop).into_str_parser();
        let addx = preceded(tuple((tag("addx"), space1)), i32).map(Op::Addx);
        let op = alt((noop, addx));
        let program = into(separated_list0(multispace1, op));
        program.anyhow(s)
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
    program.into_micro_instructions().map(move |i| {
        x += i;
        x - i
    })
}

fn draw_scan_line(line: impl Iterator<Item = i32>) -> String {
    line.zip(0..)
        .map(|(sprite_pos, i)| {
            if (sprite_pos - 1..=sprite_pos + 1).contains(&i) {
                '#'
            } else {
                '.'
            }
        })
        .join("")
}

fn part2(program: Program) -> String {
    run(program)
        .chunks(40)
        .into_iter()
        .map(draw_scan_line)
        .join("\n")
}
