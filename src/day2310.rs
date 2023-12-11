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
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(usize, usize);

struct Dim(Range<usize>, Range<usize>);

impl Pos {
    fn contains(self, other: Self) -> bool {
        (0..self.0).contains(&other.0) && (0..self.1).contains(&other.1)
    }

    fn walk(self, direction: Direction, dim: Pos) -> Option<Self> {
        let Pos(row, col) = self;
        let (row, col) = match direction {
            Direction::N => (row.checked_sub(1), Some(col)),
            Direction::E => (Some(row), col.checked_add(1)),
            Direction::S => (row.checked_add(1), Some(col)),
            Direction::W => (Some(row), col.checked_sub(1)),
        };
        if let (Some(row), Some(col)) = (row, col) {
            let pos = Pos(row, col);
            dim.contains(pos).then_some(pos)
        } else {
            None
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
        Pos(self.tiles.len(), self.tiles[0].len())
    }

    fn find_animal(&mut self) -> Pos {
        let pos = self
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col, tile)| (Pos(row, col), tile))
            })
            .find(|(_pos, &tile)| tile == Tile::Start)
            .expect("no animal")
            .0;
        self.reveal_hidden_tile(pos);
        pos
    }

    fn reveal_hidden_tile(&mut self, pos: Pos) {
        use Direction::*;
        use Tile::*;
        let dim = self.dim();
        let connections = [N, E, S, W]
            .iter()
            .map(|direction| {
                pos.walk(*direction, dim)
                    .map(|pos| {
                        self.at(pos)
                            .map(|tile| tile.directions().contains(&direction.opposite()))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .collect_vec();
        self.tiles[pos.0][pos.1] = match connections.as_slice() {
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
            Some(self.tiles[pos.0][pos.1])
        } else {
            None
        }
    }

    fn walk_from(&self, prev: Pos, pos: Pos) -> (Direction, Pos) {
        let dim = self.dim();
        self.at(pos)
            .into_iter()
            .flat_map(|tile| {
                tile.directions()
                    .into_iter()
                    .filter_map(|direction| pos.walk(direction, dim).map(|pos| (direction, pos)))
            })
            .find(|&(_direction, pos)| dim.contains(pos) && pos != prev)
            .expect("bad maze")
    }

    fn path(&mut self) -> (Vec<Pos>, Vec<Direction>) {
        let fst = self.find_animal();
        let prv = fst;
        let (direction, cur) = self.walk_from(prv, prv);
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

    // Clear out all the plumbing that doesn't belong to the cycle
    let boundary = positions.into_iter().collect::<HashSet<_>>();
    for (row, line) in maze.tiles.iter_mut().enumerate() {
        for (col, tile) in line.iter_mut().enumerate() {
            if !boundary.contains(&Pos(row, col)) {
                *tile = Tile::No;
            }
        }
    }

    use Tile::*;
    let mut c = 0; // Number of interior tiles encountered so far
    let mut interior = false; // Are we currently inside the loop?
    let mut latest_ne = false; // Have we seen NE more recently than SE?
    for row in maze.tiles {
        for tile in row {
            match (tile, interior, latest_ne) {
                (No, true, _) => c += 1,
                (NE, _, _) => latest_ne = true,
                (SE, _, _) => latest_ne = false,
                (NS, _, _) => interior = !interior,
                (SW, _, true) => interior = !interior,
                (NW, _, false) => interior = !interior,
                _ => {}
            }
        }
    }
    c
}
