use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{iterator, map, map_res, peek, value},
    multi::many_till,
    sequence::pair,
    IResult,
};

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<String>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        Ok((
            input,
            input.trim().split('\n').map(ToString::to_string).collect(),
        ))
    }

    type Output1 = u32;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        fn to_digit(input: &str) -> IResult<&str, u32> {
            map_res(anychar, |c| c.to_digit(10).ok_or(()))(input)
        }

        calc(input, to_digit)
    }

    type Output2 = u32;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        fn to_digit(input: &str) -> IResult<&str, u32> {
            alt((
                map_res(anychar, |c| c.to_digit(10).ok_or(())),
                value(1, pair(peek(tag("one")), anychar)),
                value(2, pair(peek(tag("two")), anychar)),
                value(3, pair(peek(tag("three")), anychar)),
                value(4, pair(peek(tag("four")), anychar)),
                value(5, pair(peek(tag("five")), anychar)),
                value(6, pair(peek(tag("six")), anychar)),
                value(7, pair(peek(tag("seven")), anychar)),
                value(8, pair(peek(tag("eight")), anychar)),
                value(9, pair(peek(tag("nine")), anychar)),
            ))(input)
        }

        calc(input, to_digit)
    }
}

fn calc(input: &Vec<String>, to_digit: fn(&str) -> IResult<&str, u32>) -> u32 {
    input
        .iter()
        .map(AsRef::as_ref)
        .map(|row| {
            let mut digits = iterator(row, map(many_till(anychar, to_digit), |(_, v)| v));
            let mut digits = digits.into_iter();
            let first = digits.next().expect("first digit");
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}
