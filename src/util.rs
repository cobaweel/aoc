pub use anyhow::anyhow;
pub use derive_more::{Deref, DerefMut, From, Into};
pub use itertools::Itertools;
pub use std::cmp::{max, min};
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
pub use std::fmt::{Debug, Display};
pub use std::str::FromStr;
pub use strum::EnumIter;
pub use strum::IntoEnumIterator as _;

/// Create a sensibly named test case that invokes `aoc_run_test`. For instance,
/// `aoc_test!(part1, 230101, 10)` would expand to:
///
/// ```notest
/// #[test]
/// fn test_part1_230101() {
///     aoc_run_test(part1, 230101, 10);
/// }
/// ```
macro_rules! aoc_test {
        ($part:ident, $input:expr, $val:expr) => {
            concat_idents::concat_idents!(test_name = test_, $input, _, $part {
                #[test]
                fn test_name() {
                    aoc_run_test($part, $input, $val);
                }
            });
        };
    }
pub(crate) use aoc_test;

/// This function, usually invoked by way of the `aoc_test!` macro, will:
///
/// 1. Read the input file `"input/{input_number}.txt"`
/// 2. Parse the input file into some type `T:FromStr` accepted by `process`
/// 3. Run the `process` function on the parsed input
/// 4. Assert that the result of the `process` function is equal to the `expected` value
pub fn aoc_run_test<I, T>(process: impl Fn(I) -> T, input_number: u32, expected: T)
where
    I: FromStr,
    I::Err: Debug,
    T: Eq + Debug,
{
    let input = parse_test_file(input_number);
    assert_eq!(process(input), expected);
}

pub fn parse_test_file<I>(input_number: u32) -> I
where
    I: FromStr,
    I::Err: Debug,
{
    let input_path = format!("input/{}.txt", input_number);
    let input_string = std::fs::read_to_string(input_path).expect("cannot read test data");
    input_string.parse().expect("cannot parse test data")
}

/// This module has everything needed for parsing AOC input files with the nom
/// crate. That includes most of the common combinators used for parsing a
/// `&str` input, as well as some convenience features.
pub mod aoc_nom {
    use nom::error::Error;
    pub use nom::{
        branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    };
    pub use nom::{IResult, Parser};

    /// Extension trait that is auto-implemented for any parser that operates on
    /// `&str` input.
    pub trait StrParser<'a, O>: Parser<&'a str, O, Error<&'a str>> {
        /// Specify that this `Parser` is a `StrParser`. This sometimes helps
        /// the type checker be less confused.
        fn into_str_parser(self) -> Self
        where
            Self: Sized,
        {
            self
        }

        fn parses<T>(self) -> Self
        where
            Self: Sized + StrParser<'a, T>,
        {
            self
        }

        /// Use this to eventually actually apply a complete parser to an input
        /// string. It converts the error type into one that doesn't contain
        /// references into the input, making it suitable for implementing
        /// `FromStr`, which doesn't allow such shenanigans.
        fn anyhow(mut self, s: &'a str) -> anyhow::Result<O>
        where
            Self: Sized,
        {
            let result = self.parse(s);
            let result = result.map_err(|e| e.to_owned());
            let result = result.map(|(_, o)| o)?;
            Ok(result)
        }
    }

    impl<'a, O, T> StrParser<'a, O> for T where T: Parser<&'a str, O, Error<&'a str>> {}
}

pub mod aoc {
    use itertools::Itertools;

    /// Maybe this is tucked away somewhere inside of `ndarray` already, but for
    /// the life of me, I can't find it. This transforms an iterator of
    /// iterators of `T` to an `Array2<T>`, which is especially helpful when
    /// parsing 2D grids from textual representations.
    pub fn array2<TTT, TT, T: std::fmt::Debug>(ttt: TTT) -> anyhow::Result<ndarray::Array2<T>>
    where
        TTT: IntoIterator<Item = TT>,
        TT: IntoIterator<Item = T>,
    {
        let mut shape = None;
        let array: ndarray::Array1<T> = ttt
            .into_iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.into_iter()
                    .enumerate()
                    .map(move |(col, t)| ((row, col), t))
            })
            .map(|((row, col), t)| {
                let _ = shape.insert((row + 1, col + 1));
                t
            })
            .collect();
        let shape = shape.unwrap_or((0, 0));
        let array = array.into_shape(shape)?;
        Ok(array)
    }
}

pub mod aoc_grid {
    use super::*;
    use strum::EnumIter;

    #[derive(PartialEq, Eq, Copy, Clone, Debug, EnumIter, Hash)]
    pub enum Dir {
        N,
        E,
        S,
        W,
    }

    impl Dir {
        pub fn opposite(self) -> Self {
            match self {
                Dir::N => Dir::S,
                Dir::E => Dir::W,
                Dir::S => Dir::N,
                Dir::W => Dir::E,
            }
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, From)]
    pub struct Pos(pub usize, pub usize);

    impl Pos {
        pub fn contains(self, other: Self) -> bool {
            (0..self.0).contains(&other.0) && (0..self.1).contains(&other.1)
        }

        pub fn walk(self, direction: Dir, dim: Pos) -> Option<Self> {
            let Pos(row, col) = self;
            let (row, col) = match direction {
                Dir::N => (row.checked_sub(1), Some(col)),
                Dir::E => (Some(row), col.checked_add(1)),
                Dir::S => (row.checked_add(1), Some(col)),
                Dir::W => (Some(row), col.checked_sub(1)),
            };
            if let (Some(row), Some(col)) = (row, col) {
                let pos = Pos(row, col);
                dim.contains(pos).then_some(pos)
            } else {
                None
            }
        }
    }
}
