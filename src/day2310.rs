use std::ops::Range;

use itertools::iterate;

use crate::util::*;

aoc_test!(part1, 231001, 4);
aoc_test!(part1, 231002, 8);
aoc_test!(part1, 231000, 6690);
aoc_test!(part2, 231001, 1);
aoc_test!(part2, 231003, 4);
aoc_test!(part2, 231004, 8);
aoc_test!(part2, 231005, 10);

aoc_test!(part2, 231000, 525);

#[derive(Debug, From, Clone)]
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }

    fn next(self) -> Self {
        use Direction::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }

    fn angle(self) -> isize {
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        }
    }

    fn rotation(self, prv: Direction) -> isize {
        use Direction::*;
        let mut rotation = self.angle() - prv.angle();
        if rotation == 3 {
            rotation = -1;
        }
        rotation
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(isize, isize);

struct Dim(Range<usize>, Range<usize>);

impl Pos {
    fn into_index(self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }

    fn contains(self, other: Self) -> bool {
        (0..self.0).contains(&other.0) && (0..self.1).contains(&other.1)
    }

    fn contains_in_edge(self, other: Self) -> bool {
        other.0 == 0 || other.1 == 0 || other.0 == self.0 || other.1 == self.1
    }

    fn walk(self, direction: Direction) -> Self {
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

    fn find_animal(&mut self) -> Pos {
        let mut pos = None;
        let dim = self.dim();
        for x in 0..dim.0 {
            for y in 0..dim.1 {
                let pos_ = Pos(x, y);
                if self.at(pos_) == Some(Tile::Start) {
                    let _ = pos.insert(pos_);
                }
            }
        }
        let pos = pos.expect("no animal");
        self.reveal_hidden_tile(pos);
        pos
    }

    fn reveal_hidden_tile(&mut self, pos: Pos) {
        use Direction::*;
        use Tile::*;
        let connections = [N, E, S, W]
            .iter()
            .map(|direction| {
                self.at(pos.walk(*direction))
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
        let dim = self.dim();
        if dim.contains(pos) {
            let (row, col) = pos.into_index();
            Some(self.tiles[row][col])
        } else {
            None
        }
    }

    fn walk(&self, pos: Pos) -> Vec<(Direction, Pos)> {
        let dim = self.dim();
        self.at(pos)
            .into_iter()
            .flat_map(|tile| {
                tile.directions()
                    .into_iter()
                    .map(|direction| (direction, pos.walk(direction)))
            })
            .filter(|(_direction, pos)| dim.contains(*pos))
            .collect()
    }

    fn walk_from(&self, prev: Pos, pos: Pos) -> (Direction, Pos) {
        let (pos,) = self
            .walk(pos)
            .into_iter()
            .filter(|&(_direction, pos)| pos != prev)
            .collect_tuple()
            .expect("bad maze");
        pos
    }

    fn path(&mut self) -> (Vec<Pos>, Vec<Direction>) {
        let fst = self.find_animal();
        let prv = fst;
        let (direction, cur) = self.walk(prv)[0];
        let (positions, directions): (Vec<Pos>, Vec<Direction>) =
            itertools::iterate((prv, direction, cur), |&(prv, _direction, cur)| {
                let (direction, nxt) = self.walk_from(prv, cur);
                (cur, direction, nxt)
            })
            .take_while_inclusive(move |&(_prv, _direction, cur)| cur != fst)
            .map(|(prv, direction, _cur)| (prv, direction))
            .unzip();
        (positions, directions)
    }
}

fn part1(mut maze: Maze) -> usize {
    let (positions, _directions) = maze.path();
    positions.len() / 2
}

fn part2(mut maze: Maze) -> usize {
    let (positions, _) = maze.path();
    let boundary = positions
        .into_iter()
        .map(|pos| pos.into_index())
        .collect::<HashSet<_>>();
    for (row, line) in maze.tiles.iter_mut().enumerate() {
        for (col, tile) in line.iter_mut().enumerate() {
            if !boundary.contains(&(row, col)) {
                *tile = Tile::No;
            }
        }
    }

    use Tile::*;
    let mut c = 0;
    let mut dc = 0;
    let mut down = No;
    for row in maze.tiles {
        for tile in row {
            match (down, tile) {
                (_, No) => c += dc,
                (_, NS) => dc = 1 - dc,
                (_, NE) => down = NE,
                (_, SE) => down = SE,
                (NE, SW) => dc = 1 - dc,
                (SE, NW) => dc = 1 - dc,
                _ => {}
            }
        }
    }
    c
}
