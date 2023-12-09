use std::{fmt::Debug, str::FromStr};

pub fn test<T>(process: impl Fn(&str) -> T, input_number: u32, expected: T)
where
    T: Eq + Debug,
{
    let input_path = format!("input/{input_number}.txt");
    let input_string = std::fs::read_to_string(input_path).expect("cannot read test data");
    let output = process(input_string.as_ref());
    assert_eq!(output, expected);
}

pub fn parse_and_test<I, T>(process: impl Fn(I) -> T, input_number: u32, expected: T)
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

pub mod parse_with_nom {
    use nom::error::Error;
    pub use nom::{
        branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    };
    pub use nom::{IResult, Parser};

    /// Extension trait that is auto-implemented for any parser that operates on
    /// `&str` input.
    pub trait StrParser<'a, O>: Parser<&'a str, O, Error<&'a str>> {
        /// This does nothing at runtime, but it indicates to the typechecker
        /// that this `Parser` is in fact a `StrParser`, which shouldn't be
        /// necessary for complete code (since this information is usually
        /// back-propagated from the point where the parser is eventually
        /// applied to a `&str`), but it can be very helpful for debugging.
        fn id(self) -> Self
        where
            Self: Sized,
        {
            self
        }

        /// Use this to eventually actually apply a complete parser to an input
        /// string. It converts the error type into one that doesn't contain
        /// references into the input, making it suitable for implementing
        /// `FromStr`.
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

    pub trait Muncher<O> {
        fn munch(self) -> anyhow::Result<O>;
    }

    impl<'a, O> Muncher<O> for nom::IResult<&'a str, O> {
        fn munch(self) -> anyhow::Result<O> {
            let result = self.map_err(|e| e.to_owned());
            let result = result.map(|(_, o)| o);
            let result = result?;
            Ok(result)
        }
    }
}
