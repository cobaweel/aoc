pub use anyhow::anyhow;
pub use derive_more::{From, Into};
pub use itertools::Itertools;
pub use std::cmp::{max, min};
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
pub use std::fmt::{Debug, Display};
pub use std::str::FromStr;

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
    let input_path = format!("input/{input_number}.txt");
    let input_string = std::fs::read_to_string(input_path).expect("cannot read test data");
    let input = input_string.parse().expect("cannot parse test data");
    assert_eq!(process(input), expected);
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
        TTT: Iterator<Item = TT>,
        TT: Iterator<Item = T>,
    {
        let mut shape = None;
        let array: ndarray::Array1<T> = ttt
            .enumerate()
            .flat_map(|(row, line)| line.enumerate().map(move |(col, t)| ((row, col), t)))
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
