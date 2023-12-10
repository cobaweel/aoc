use crate::util::*;

use dyn_iter::DynIter;
use dyn_iter::IntoDynIterator as _;

// aoc_test!(part1, 220801, 21);
// aoc_test!(part1, 220800, 1719);
// aoc_test!(part2, 220801, 0);
// aoc_test!(part2, 220800, 0);

type Grid<T> = BTreeMap<Position, T>;
type Position = (usize, usize);

type ComboRange<'a> = DynIter<'a, usize>;

fn bwd<'a>(lb: usize, ub: usize) -> ComboRange<'a> {
    (lb..ub).rev().into_dyn_iter()
}
fn fwd<'a>(lb: usize, ub: usize) -> ComboRange<'a> {
    (lb..ub).into_dyn_iter()
}
fn jst<'a>(x: usize) -> ComboRange<'a> {
    (x..=x).into_dyn_iter()
}

fn bounds<T>(grid: &Grid<T>) -> (usize, usize) {
    grid.last_key_value()
        .map(|((x, y), _)| (x + 1, y + 1))
        .unwrap_or((0, 0))
}

fn print_grid<D: Display>((xx, yy): Position, tx: impl Fn((usize, usize)) -> D) {
    for x in 0..xx {
        for y in 0..yy {
            print!("{}", tx((x, y)));
        }
        println!();
    }
}

struct Forest {
    trees: BTreeMap<(usize, usize), i32>,
}

impl Forest {
    fn parse(input: &str) -> Self {
        let trees = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap_or(0))
                    .enumerate()
                    .map(move |(y, c)| ((x, y), c))
            })
            .collect::<BTreeMap<_, _>>();
        let forest = Forest { trees };
        print_grid(forest.bounds(), |xy| forest.get(&xy));
        forest
    }

    fn get(&self, position: &Position) -> i32 {
        *self.trees.get(position).unwrap_or(&0)
    }

    fn bounds(&self) -> Position {
        bounds(&self.trees)
    }

    fn n_visible(&self) -> usize {
        use itertools::chain;
        let mut visible = BTreeSet::new();
        let (xx, yy) = self.bounds();
        let sight_lines = chain!(
            fwd(0, xx).map(|x| jst(x).zip(fwd(0, yy))),
            fwd(0, xx).map(|x| jst(x).zip(bwd(0, yy))),
            fwd(0, yy).map(|y| fwd(0, xx).zip(jst(y))),
            fwd(0, yy).map(|y| bwd(0, xx).zip(jst(y))),
        );
        for xys in sight_lines {
            let mut tallest = -1;
            for xy in xys {
                let this: i32 = self.get(&xy);
                if this > tallest {
                    tallest = this;
                    visible.insert(xy);
                }
            }
        }
        print_grid(self.bounds(), |xy| checkbox(visible.contains(&xy)));
        visible.len()
    }

    fn view(&self, (_x, _y): Position) -> usize {
        use itertools::chain;
        todo!()
    }
}

fn checkbox(v: bool) -> &'static str {
    if v {
        "☒ "
    } else {
        "☐ "
    }
}

fn part1(input: &str) -> usize {
    Forest::parse(input).n_visible()
}
