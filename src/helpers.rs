use std::str::FromStr;
use std::{fmt::Debug, ops::RangeFrom};

use nom::{
    character::complete::digit1, combinator::map, AsChar, IResult, InputIter, InputTakeAtPosition,
    ParseTo, Slice,
};

pub fn parse_digit<I, T>(error_message: &'static str) -> impl FnMut(I) -> IResult<I, T>
where
    I: Slice<RangeFrom<usize>> + InputIter + ParseTo<T> + InputTakeAtPosition,
    <I as InputIter>::Item: AsChar,
    <I as InputTakeAtPosition>::Item: AsChar,
    T: FromStr,
    T::Err: Debug,
{
    map(digit1, |n: I| n.parse_to().expect(error_message))
}
