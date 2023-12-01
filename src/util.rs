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
    pub use nom::{
        branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    };
    pub use nom::{IResult, Parser};

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
