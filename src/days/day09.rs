use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::{
    days::Day,
    helpers::{parse_digit, parse_ndigit},
};

pub struct Day09;

impl Day for Day09 {
    type Input = Vec<Vec<isize>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "0 3 6 9 12 15
        // 1 3 6 10 15 21
        // 10 13 16 21 30 45";

        all_consuming(separated_list1(
            preceded(line_ending, multispace0),
            separated_list1(
                tag(" "),
                alt((parse_digit("num"), parse_ndigit("negative num"))),
            ),
        ))(input)
    }

    type Output1 = isize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.into_iter().map(extrapolate_end).sum()
    }

    type Output2 = isize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.into_iter().map(extrapolate_start).sum()
    }
}

fn extrapolate_end(row: &Vec<isize>) -> isize {
    let differences = calculate_differences(&row);

    let next_difference = if differences.iter().all(|n| *n == 0) {
        0
    } else {
        extrapolate_end(&differences)
    };

    row.last().unwrap() + next_difference
}

fn extrapolate_start(row: &Vec<isize>) -> isize {
    let differences = calculate_differences(&row);

    let previous_difference = if differences.iter().all(|n| *n == 0) {
        0
    } else {
        extrapolate_start(&differences)
    };

    row.first().unwrap() - previous_difference
}

fn calculate_differences(row: &Vec<isize>) -> Vec<isize> {
    let mut row_iter = row.iter();
    let first = row_iter.next().copied();
    row_iter
        .scan(first, |prev, curr| {
            prev.as_mut().map(|prev| {
                let val = *curr - *prev;
                *prev = *curr;
                val
            })
        })
        .collect_vec()
}
