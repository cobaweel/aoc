use crate::util::*;

use ndarray::Array2;

aoc_test!(part1, 220801, 21);
aoc_test!(part1, 220800, 1719);
aoc_test!(part2, 220801, 8);
aoc_test!(part2, 220800, 590824);

type Pos = (usize, usize);
type Height = i32;

struct Map {
    heights: Array2<Height>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = |c| (u32::from(c) - u32::from('0')) as Height;
        let heights = s.lines().map(|line| line.chars().map(height));
        let heights = aoc::array2(heights)?;
        Ok(Map { heights })
    }
}

#[derive(Clone, Copy, EnumIter, Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn part1(map: Map) -> usize {
    map.n_visible_from_edge()
}

fn part2(map: Map) -> usize {
    map.max_scenic_score()
}

impl Map {
    /// The maximum scenic score is the highest of the scenic scores of all trees
    fn max_scenic_score(&self) -> usize {
        self.heights
            .indexed_iter()
            .map(|(pos, height)| self.scenic_score(pos, *height))
            .max()
            .unwrap_or(0)
    }

    /// The scenic score for a tree is the product of its viewing distances in all 4 directions
    fn scenic_score(&self, pos: Pos, height: Height) -> usize {
        Dir::iter()
            .map(|dir| self.viewing_distance(pos, height, dir))
            .product()
    }

    /// The viewing distance from some tree in some direction is the length of its view
    fn viewing_distance(&self, pos: Pos, height: Height, dir: Dir) -> usize {
        self.view(pos, height, dir).count()
    }

    /// To find the view from a given tree in a given direction, keep walking until we hit a tree
    /// that is at least as tall as the first, or the edge
    fn view(&self, pos: Pos, height: Height, dir: Dir) -> impl Iterator<Item = Pos> + '_ {
        self.path(pos, height, dir)
            .skip(1)
            .take_while_inclusive(move |&(_, h)| h < height)
            .map(|(pos, _)| pos)
    }

    /// From a given starting position, the path in some direction is the sequence of heights of
    /// the trees you pass moving in that direction
    fn path(&self, pos: Pos, height: Height, dir: Dir) -> impl Iterator<Item = (Pos, Height)> + '_ {
        let first = std::iter::once((pos, height));
        let rest = std::iter::repeat(()).scan(pos, move |cur, _| {
            if let Some(nxt) = Self::next(dir, *cur) {
                *cur = nxt;
                self.heights.get(nxt).map(|h| (nxt, *h))
            } else {
                None
            }
        });
        first.chain(rest)
    }

    /// For a given starting position, compute the next position in some direction
    fn next(dir: Dir, (row, col): Pos) -> Option<Pos> {
        if let (Some(row), Some(col)) = match dir {
            Dir::Left => (Some(row), col.checked_sub(1)),
            Dir::Right => (Some(row), col.checked_add(1)),
            Dir::Up => (row.checked_sub(1), Some(col)),
            Dir::Down => (row.checked_add(1), Some(col)),
        } {
            Some((row, col))
        } else {
            None
        }
    }

    /// To find the view from an edge position in a given direction, walk all the way to the other
    /// end, reporting every tree that is taller than the tallest seen before it
    fn edge_view(&self, fst: Pos, dir: Dir) -> impl Iterator<Item = Pos> + '_ {
        let height = self.heights[fst];
        let view = self.path(fst, height, dir);
        view.scan(-1, |tallest, (pos, h)| {
            if h > *tallest {
                *tallest = h;
                Some(Some(pos))
            } else {
                Some(None)
            }
        })
        .flatten()
    }

    /// To count the number of trees that are visible from some edge, we gather up all the
    /// `edge_view`s and count the number of distinct trees they contain
    fn n_visible_from_edge(&self) -> usize {
        let (rows, cols) = self.heights.dim();
        itertools::chain!(
            (0..rows).flat_map(|row| self.edge_view((row, 0), Dir::Right)),
            (0..rows).flat_map(|row| self.edge_view((row, cols - 1), Dir::Left)),
            (0..cols).flat_map(|col| self.edge_view((0, col), Dir::Down)),
            (0..cols).flat_map(|col| self.edge_view((rows - 1, col), Dir::Up))
        )
        .unique()
        .count()
    }
}
