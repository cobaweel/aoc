use ndarray::Array2;

use crate::util::*;

aoc_test!(expand_2, 231101, 374);
aoc_test!(expand_2, 231100, 9734203);
aoc_test!(expand_1e1, 231101, 1030);
aoc_test!(expand_1e2, 231101, 8410);
aoc_test!(expand_1e6, 231100, 568914596391);

struct Chart {
    galaxies: Array2<bool>,
}

impl FromStr for Chart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = aoc::array2(s.lines().map(|line| line.chars().map(|c| c == '#')))?;
        Ok(Chart { galaxies })
    }
}

fn expand_2(chart: Chart) -> usize {
    compute(chart, 2)
}

fn expand_1e1(chart: Chart) -> usize {
    compute(chart, 10)
}

fn expand_1e2(chart: Chart) -> usize {
    compute(chart, 100)
}

fn expand_1e6(chart: Chart) -> usize {
    compute(chart, 1000000)
}

// Solve the problem with a universe expansion factor of `k`.
fn compute(Chart { galaxies }: Chart, k: usize) -> usize {
    // Gather up the indices of empty rows and columns into ordered sets.
    let empty_rows: BTreeSet<usize> = galaxies
        .rows()
        .into_iter()
        .enumerate()
        .flat_map(|(i, a)| a.into_iter().all(|p| !p).then_some(i))
        .collect();
    let empty_cols: BTreeSet<usize> = galaxies
        .columns()
        .into_iter()
        .enumerate()
        .flat_map(|(i, a)| a.into_iter().all(|p| !p).then_some(i))
        .collect();

    // Transform the grid representation of the problem into a sequence of
    // coordinate, one for each galaxy
    let galaxy_positions = galaxies
        .indexed_iter()
        .flat_map(|(pos, p)| p.then_some(pos));

    // There's a handy library function to enumerate all combinations of two
    // galaxies!
    let galaxy_position_pairs = galaxy_positions.tuple_combinations();

    // And finally, for each pair of galaxies, compute the Manhattan distance,
    // adding in `k-1` times the number of empty rows and columns between the
    // two.
    galaxy_position_pairs
        .map(|((row0, col0), (row1, col1))| {
            let row = min(row0, row1)..=max(row0, row1);
            let col = min(col0, col1)..=max(col0, col1);
            let d_row = row.end() - row.start() + (k - 1) * empty_rows.range(row).count();
            let d_col = col.end() - col.start() + (k - 1) * empty_cols.range(col).count();
            d_row + d_col
        })
        .sum()
}
