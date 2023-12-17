use crate::util::*;
use aoc_grid::*;
use itertools::chain;
use ndarray::Array2;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::ParallelIterator as _;
use winnow::combinator::fail;

aoc_test!(part1, 231601, 46);
aoc_test!(part1, 231600, 6902);
aoc_test!(part2, 231601, 51);
aoc_test!(part2, 231600, 7697);

#[derive(Debug)]
enum Tile {
    Forward,
    Backward,
    Upright,
    Flat,
    Empty,
}

use Tile::*;

#[derive(Deref)]
struct Grid(Array2<Tile>);

fn part1(grid: Grid) -> usize {
    grid.scan(Pos(0, 0), Dir::E)
}

fn part2(grid: Grid) -> usize {
    use Dir::*;
    let (rows, cols) = grid.dim();
    rayon::iter::empty()
        .chain((0..rows).into_par_iter().map(|row| (Pos(row, 0), E)))
        .chain((0..rows).into_par_iter().map(|row| (Pos(row, cols - 1), W)))
        .chain((0..cols).into_par_iter().map(|col| (Pos(0, col), S)))
        .chain((0..cols).into_par_iter().map(|col| (Pos(rows - 1, col), N)))
        .map(|(pos, dir)| grid.scan(pos, dir))
        .max()
        .unwrap_or(0)
}

impl Grid {
    fn at(&self, Pos(r, c): Pos) -> Option<&Tile> {
        self.get((r, c))
    }

    fn scan(&self, pos: Pos, dir: Dir) -> usize {
        let mut work = vec![(pos, dir)];
        let dim = Pos::from(self.dim());
        let mut seen: HashSet<(Pos, Dir)> = HashSet::new();
        use Dir::*;
        while let Some((pos, dir)) = work.pop() {
            if seen.contains(&(pos, dir)) {
                continue;
            } else {
                seen.insert((pos, dir));
            }
            let dirs_ = match (self.at(pos), dir) {
                (Some(Forward), N) => vec![E],
                (Some(Forward), E) => vec![N],
                (Some(Forward), S) => vec![W],
                (Some(Forward), W) => vec![S],
                (Some(Backward), N) => vec![W],
                (Some(Backward), W) => vec![N],
                (Some(Backward), E) => vec![S],
                (Some(Backward), S) => vec![E],
                (Some(Upright), E) => vec![N, S],
                (Some(Upright), W) => vec![N, S],
                (Some(Flat), N) => vec![E, W],
                (Some(Flat), S) => vec![E, W],
                (Some(_), d) => vec![d],
                (None, _) => vec![],
            };
            for dir_ in dirs_ {
                if let Some(pos_) = pos.walk(dir_, dim) {
                    work.push((pos_, dir_));
                }
            }
        }
        seen.into_iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '/' => Ok(Forward),
                        '\\' => Ok(Backward),
                        '|' => Ok(Upright),
                        '-' => Ok(Flat),
                        '.' => Ok(Empty),
                        _ => Err(anyhow!("bad grid")),
                    })
                    .collect::<Result<Vec<Tile>, _>>()
            })
            .collect::<Result<Vec<Vec<Tile>>, _>>()?;
        let tiles = aoc::array2(tiles)?;
        Ok(Grid(tiles))
    }
}
