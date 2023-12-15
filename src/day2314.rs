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
    platform.roll_n();
    platform.total_load()
}

fn part2(mut platform: Platform) -> usize {
    let mut i_by_platform = HashMap::new();
    let mut i = 0;
    let n = 1000000000;
    while i < n {
        platform.cycle();
        if let Some(j) = i_by_platform.insert(platform.clone(), i) {
            i = n - (n - i) % (i - j) + 1;
        } else {
            i += 1;
        }
    }
    platform.total_load()
}

type Pos = (usize, usize);
type Mov = (isize, isize);

impl Platform {
    fn total_load(&self) -> usize {
        let array = self.0.slice(s![..;-1, ..]);
        array
            .indexed_iter()
            .map(|((row, _), tile)| {
                if matches!(tile, Tile::Round) {
                    row + 1
                } else {
                    0
                }
            })
            .sum()
    }

    fn walk((r, c): (usize, usize), (dr, dc): (isize, isize)) -> Option<(usize, usize)> {
        if let (Some(r), Some(c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
            Some((r, c))
        } else {
            None
        }
    }

    fn cycle(&mut self) {
        Self::roll(self.0.view_mut());
        Self::roll(self.0.view_mut().reversed_axes());
        Self::roll(self.0.slice_mut(s![..;-1, ..]));
        Self::roll(self.0.slice_mut(s![.., ..;-1]).reversed_axes());
    }

    fn roll_n(&mut self) {
        Self::roll(self.0.view_mut());
    }

    fn roll(mut array: ArrayViewMut2<Tile>) {
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
