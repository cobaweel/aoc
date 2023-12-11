use itertools::izip;
use ndarray::{Array1, Array2, Shape};

use crate::util::*;

aoc_test!(part1, 221201, 31);
aoc_test!(part1, 221200, 412);
aoc_test!(part2, 221201, 29);
aoc_test!(part2, 221200, 402);
struct Chart {
    elevations: Array2<u8>,
    src: (usize, usize),
    dst: (usize, usize),
}

impl FromStr for Chart {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use aoc_nom::*;
        let mut src = None;
        let mut dst = None;
        let mut cols = None;
        let mut rows = None;
        let mut elevations = Vec::with_capacity(s.len());

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let _ = cols.insert(col + 1);
                let _ = rows.insert(row + 1);
                let c = match c {
                    'S' => {
                        let _ = src.insert((row, col));
                        'a'
                    }
                    'E' => {
                        let _ = dst.insert((row, col));
                        'z'
                    }
                    c => c,
                };
                let elevation = u32::from(c) - u32::from('a');
                elevations.push(elevation as u8);
            }
        }

        let rows = rows.ok_or(anyhow!("no rows"))?;
        let cols = cols.ok_or(anyhow!("no cols"))?;
        let src = src.ok_or(anyhow!("no source"))?;
        let dst = dst.ok_or(anyhow!("no destination"))?;
        let elevations: Array2<u8> = Array1::from_vec(elevations).into_shape((rows, cols))?;
        let chart = Chart {
            elevations,
            src,
            dst,
        };
        Ok(chart)
    }
}

fn part1(chart: Chart) -> usize {
    *distances(&chart)
        .get(chart.src)
        .expect("can't get there from here")
}

fn part2(chart: Chart) -> usize {
    let distances = distances(&chart);
    chart
        .elevations
        .indexed_iter()
        .filter(|(_, &elevation)| elevation == 0u8)
        .map(|(pos, _)| distances[pos])
        .min()
        .expect("can't get there from here")
}

fn distances(chart: &Chart) -> Array2<usize> {
    let elevations = &chart.elevations;
    let mut work = VecDeque::from([(chart.dst, 0)]);
    let mut seen = Array2::from_elem(elevations.raw_dim(), false);
    let mut distances = Array2::from_elem(elevations.raw_dim(), usize::MAX);
    while let Some((pos @ (row, col), distance)) = work.pop_front() {
        if !seen[pos] {
            seen[pos] = true;
            distances[pos] = distance;
            let neighbors = [
                (row.checked_sub(1), Some(col)),
                (row.checked_add(1), Some(col)),
                (Some(row), col.checked_sub(1)),
                (Some(row), col.checked_add(1)),
            ];
            let elevation = elevations[pos];
            for pos_ in neighbors {
                if let (Some(row_), Some(col_)) = pos_ {
                    if let Some(&elevation_) = elevations.get((row_, col_)) {
                        if elevation_ + 1 >= elevation {
                            work.push_back(((row_, col_), distance + 1));
                        }
                    }
                }
            }
        }
    }
    distances
}
