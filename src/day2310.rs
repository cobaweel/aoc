use itertools::iterate;

use crate::util::*;

aoc_test!(part1, 231001, 4);
aoc_test!(part1, 231002, 8);
aoc_test!(part1, 231000, 6690);

#[derive(Debug, From)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    No,
    Start,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        use Direction::*;
        match self {
            Tile::NS => vec![N, S],
            Tile::EW => vec![E, W],
            Tile::NE => vec![N, E],
            Tile::NW => vec![N, W],
            Tile::SW => vec![S, W],
            Tile::SE => vec![S, E],
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Pos(isize, isize);

impl Pos {
    fn into_index(self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }

    fn contained_by(self, dim: &Self) -> bool {
        (0..dim.0).contains(&self.0) && (0..dim.1).contains(&self.1)
    }

    fn walk(self, direction: &Direction) -> Self {
        let Pos(row, col) = self;
        match direction {
            Direction::N => Pos(row - 1, col),
            Direction::E => Pos(row, col + 1),
            Direction::S => Pos(row + 1, col),
            Direction::W => Pos(row, col - 1),
        }
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        use Tile::*;
        let tile = alt((
            tag("|").map(|_| NS),
            tag("-").map(|_| EW),
            tag("L").map(|_| NE),
            tag("J").map(|_| NW),
            tag("7").map(|_| SW),
            tag("F").map(|_| SE),
            tag(".").map(|_| No),
            tag("S").map(|_| Start),
        ));
        let maze = separated_list1(multispace1, many1(tile)).map(Maze::from);
        maze.anyhow(s)
    }
}

impl Maze {
    fn dim(&self) -> Pos {
        Pos(self.tiles.len() as isize, self.tiles[0].len() as isize)
    }

    fn find_animal(&mut self) -> Option<Pos> {
        let mut out = None;
        let dim = self.dim();
        for x in 0..dim.0 {
            for y in 0..dim.1 {
                let pos_ = Pos(x, y);
                if self.at(pos_) == Some(Tile::Start) {
                    let _ = out.insert(pos_);
                }
            }
        }
        if let Some(pos) = out {
            self.reveal_hidden_tile(pos);
        }
        out
    }

    fn reveal_hidden_tile(&mut self, pos: Pos) {
        use Direction::*;
        use Tile::*;
        let connections = [N, E, S, W]
            .iter()
            .map(|direction| {
                self.at(pos.walk(direction))
                    .map(|tile| tile.directions().contains(&direction.opposite()))
                    .unwrap_or(false)
            })
            .collect_vec();
        let (row, col) = pos.into_index();
        self.tiles[row][col] = match connections.as_slice() {
            // N     E     S      W
            [true, true, false, false] => NE,
            [true, false, true, false] => NS,
            [true, false, false, true] => NW,
            [false, true, true, false] => SE,
            [false, true, false, true] => EW,
            [false, false, true, true] => SW,
            _ => No,
        };
    }

    fn at(&self, pos: Pos) -> Option<Tile> {
        if pos.contained_by(&self.dim()) {
            let (row, col) = pos.into_index();
            Some(self.tiles[row][col])
        } else {
            None
        }
    }

    fn walk(&self, pos: Pos) -> Vec<Pos> {
        self.at(pos)
            .into_iter()
            .flat_map(|tile| {
                tile.directions()
                    .into_iter()
                    .map(|direction| pos.walk(&direction))
            })
            .collect()
    }

    fn walk_from(&self, prev: Pos, pos: Pos) -> Pos {
        let (pos,) = self
            .walk(pos)
            .into_iter()
            .filter(|&pos| pos != prev)
            .collect_tuple()
            .expect("bad maze");
        pos
    }

    // fn path(&self) -> Vec<Pos> {
    //     let fst = self.find_animal().expect("no animal");
    //     let prv = fst;
    //     let cur = self.walk(prv)[0];
    //     let path = itertools::iterate((prv, cur), |&(prv, cur)| (cur, self.walk_from(prv, cur)))
    //         .take_while_inclusive(move |&(_prv, cur)| cur != fst)
    //         .map(|(prv, _cur)| prv)
    //         .collect_vec();

    // }
}

fn part1(mut maze: Maze) -> u32 {
    let start = maze.find_animal().expect("no animal");
    let (rows, cols) = maze.dim().into_index();
    let mut min_distances = vec![vec![None; cols]; rows];
    for pos in maze.walk(start) {
        let mut prv = start;
        let mut cur = pos;
        let mut distance_walked = 1;
        while cur != start {
            let (row, col) = cur.into_index();
            let min_distance = min_distances[row][col].unwrap_or(u32::MAX);
            let min_distance = std::cmp::min(min_distance, distance_walked);
            let _ = min_distances[row][col].insert(min_distance);
            (prv, cur) = (cur, maze.walk_from(prv, cur));
            distance_walked += 1;
        }
    }
    min_distances
        .into_iter()
        .flat_map(|row| row.into_iter().map(|x| x.unwrap_or(0)))
        .max()
        .expect("empty maze")
}
