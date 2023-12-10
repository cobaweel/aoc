use crate::util::*;

aoc_parse_and_test!(part1, 230301, 4361);
aoc_parse_and_test!(part1, 230300, 525181);
aoc_parse_and_test!(part2, 230301, 467835);
aoc_parse_and_test!(part2, 230300, 84289137);

struct Input {
    grid: ndarray::Array2<char>,
    numbers: Vec<Number>,
}

struct Number {
    x: usize,
    y_start: usize,
    y_end: usize,
    n: u32,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = parse_grid(s)?;
        let numbers = parse_numbers(s)?;
        Ok(Input { grid, numbers })
    }
}

fn parse_numbers(s: &str) -> Result<Vec<Number>, anyhow::Error> {
    let mut out = vec![];
    let re = regex::Regex::new(r"\d+")?;
    for (x, line) in s.lines().enumerate() {
        for m in re.find_iter(line) {
            out.push(Number {
                x,
                y_start: m.start(),
                y_end: m.end(),
                n: m.as_str().parse()?,
            });
        }
    }
    Ok(out)
}

fn parse_grid(s: &str) -> Result<ndarray::Array2<char>, anyhow::Error> {
    // Surely there's a more elegant way to do this...
    let mut xx = 0;
    let mut yy = 0;
    let chars = s
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            xx = std::cmp::max(xx, x + 1);
            yy = std::cmp::max(yy, line.len());
            line.chars()
        })
        .collect_vec();
    let chars = ndarray::Array::from_vec(chars);
    let chars = chars.into_shape((xx, yy))?;
    Ok(chars)
}

fn part1(Input { grid, numbers }: Input) -> u32 {
    numbers
        .into_iter()
        .flat_map(|number| {
            margin(&number, &grid)
                .any(|(_, c)| !"0123456789.".contains(*c))
                .then_some(number.n)
        })
        .sum()
}

fn part2(Input { grid, numbers }: Input) -> u32 {
    numbers
        .into_iter()
        .flat_map(|number| {
            margin(&number, &grid)
                .filter_map(|(i, c)| (c == &'*').then_some((i, number.n)))
                .collect_vec()
        })
        .into_group_map()
        .values()
        .filter_map(|ns| ns.iter().collect_tuple().map(|(a, b)| a * b))
        .sum()
}

fn margin<'a, T>(
    number: &'a Number,
    array: &'a ndarray::Array2<T>,
) -> impl Iterator<Item = ((usize, usize), &'a T)> {
    let (xx, yy) = array.dim();
    let xx_hi = xx.saturating_sub(1);
    let yy_hi = yy.saturating_sub(1);
    let x_lo = number.x.saturating_sub(1).clamp(0, xx_hi);
    let x_hi = number.x.saturating_add(1).clamp(0, xx_hi);
    let y_lo = number.y_start.saturating_sub(1).clamp(0, yy_hi);
    let y_hi = number.y_end.saturating_add(1).clamp(0, yy_hi);
    itertools::iproduct!(x_lo..=x_hi, y_lo..y_hi).flat_map(|i| array.get(i).map(|t| (i, t)))
}
