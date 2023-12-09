use std::{fmt::Debug, ops::RangeFrom};
use std::{ops::RangeTo, str::FromStr};

use nom::{
    character::complete::{char, digit1},
    combinator::{map, recognize},
    sequence::preceded,
    AsChar, IResult, InputIter, InputTakeAtPosition, Offset, ParseTo, Slice,
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
pub fn parse_ndigit<I, T>(error_message: &'static str) -> impl FnMut(I) -> IResult<I, T>
where
    I: Slice<RangeFrom<usize>>
        + InputIter
        + ParseTo<T>
        + InputTakeAtPosition
        + Clone
        + Offset
        + Slice<RangeTo<usize>>,
    <I as InputIter>::Item: AsChar,
    <I as InputTakeAtPosition>::Item: AsChar,
    T: FromStr,
    T::Err: Debug,
{
    map(recognize(preceded(char('-'), digit1)), |n: I| {
        n.parse_to().expect(error_message)
    })
}
