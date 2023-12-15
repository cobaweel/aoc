use ndarray::{s, Array2, ArrayViewMut2};

use crate::util::*;

#[derive(Hash, Clone, Eq, PartialEq)]
struct Platform(Array2<Tile>);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Tile {
    Round,
    Cube,
    Empty,
}

aoc_test!(part1, 231401, 136);
aoc_test!(part1, 231400, 113486);
aoc_test!(part2, 231401, 64);
aoc_test!(part2, 231400, 104409);

fn part1(mut platform: Platform) -> usize {
    platform.roll();
    platform.total_load()
}

fn part2(mut platform: Platform) -> usize {
    let mut i_by_platform = HashMap::new();
    let mut i = 0;
    let n = 1000000000;
    while i < n {
        platform.cycle();
        if let Some(j) = i_by_platform.insert(platform.clone(), i) {
            // We have detected a cycle, and may as well fast forward.
            i = n - (n - i) % (i - j) + 1;
        } else {
            i += 1;
        }
    }
    platform.total_load()
}

enum Dir {
    N,
    W,
    S,
    E,
}

impl Platform {
    /// Compute the total load on the north support beam
    fn total_load(&self) -> usize {
        let upside_down = &self.0.slice(s![..;-1, ..]);
        upside_down
            .indexed_iter()
            .filter(|(_, tile)| matches!(tile, Tile::Round))
            .map(|((row, _), _)| row + 1)
            .sum()
    }

    /// View the array of tiles from a given direction
    fn view(&mut self, dir: Dir) -> ArrayViewMut2<Tile> {
        match dir {
            Dir::N => self.0.slice_mut(s![.., ..]),
            Dir::W => self.0.slice_mut(s![.., ..]).reversed_axes(),
            Dir::S => self.0.slice_mut(s![..;-1, ..]),
            Dir::E => self.0.slice_mut(s![.., ..;-1]).reversed_axes(),
        }
    }

    /// Tilt the platform in all 4 directions, starting from N, widdershins
    fn cycle(&mut self) {
        self.tilt(Dir::N);
        self.tilt(Dir::W);
        self.tilt(Dir::S);
        self.tilt(Dir::E);
    }

    /// Tilt the platform north
    fn roll(&mut self) {
        self.tilt(Dir::N);
    }

    /// Tilt the platform in the given direction
    fn tilt(&mut self, dir: Dir) {
        let mut array = self.view(dir);
        use Tile::*;
        for mut column in array.columns_mut() {
            for i in 1..column.len() {
                let mut i = i;
                while i > 0 && matches!(column[i - 1], Empty) && matches!(column[i], Round) {
                    column.swap(i, i - 1);
                    i -= 1;
                }
            }
        }
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cs = s.lines().map(|line| {
            line.chars().map(|c| match c {
                'O' => Tile::Round,
                '#' => Tile::Cube,
                '.' => Tile::Empty,
                _ => panic!(),
            })
        });
        let grid = aoc::array2(cs);
        Ok(Platform(grid?))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Round => 'O',
                Tile::Cube => '#',
                Tile::Empty => '.',
            }
        )
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.rows() {
            for tile in row.into_iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
