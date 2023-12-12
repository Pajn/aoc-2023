use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, multispace1},
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{days::Day, helpers::parse_digit};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Operational,
    Damaged,
    Unknown,
}

pub struct Day12;

impl Day for Day12 {
    type Input = Vec<(Vec<State>, Vec<usize>)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "???.### 1,1,3
        // .??..??...?##. 1,1,3
        // ?#?#?#?#?#?#?#? 1,3,1,6
        // ????.#...#... 4,1,1
        // ????.######..#####. 1,6,5
        // ?###???????? 3,2,1";

        all_consuming(separated_list1(
            multispace1,
            tuple((
                many1(alt((
                    value(State::Operational, char('.')),
                    value(State::Damaged, char('#')),
                    value(State::Unknown, char('?')),
                ))),
                preceded(
                    multispace1,
                    separated_list1(char(','), parse_digit("group")),
                ),
            )),
        ))(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        sum_combinations(input.into_iter())
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let unfolded = input
            .into_iter()
            .map(|(springs, groups)| {
                let springs = [
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                    &[State::Unknown],
                    &springs[..],
                ]
                .concat();

                let groups = [
                    &groups[..],
                    &groups[..],
                    &groups[..],
                    &groups[..],
                    &groups[..],
                ]
                .concat();

                (springs, groups)
            })
            .collect_vec();
        sum_combinations(unfolded.iter())
    }
}

fn sum_combinations<'a, I>(records: I) -> usize
where
    I: Iterator<Item = &'a (Vec<State>, Vec<usize>)>,
{
    records
        .map(|(springs, groups)| {
            let padded = [&[State::Operational], &springs[..], &[State::Operational]].concat();
            let damaged = springs
                .into_iter()
                .enumerate()
                .filter_map(|(index, s)| (*s == State::Damaged).then(|| index))
                .collect_vec();
            let variants = groups
                .iter()
                .enumerate()
                .map(|(index, &group)| {
                    let skip_start = &groups[..index];
                    let skip_start = skip_start.iter().sum::<usize>() + skip_start.len();
                    let skip_end = &groups[(index + 1)..];
                    let skip_end = skip_end.iter().sum::<usize>() + skip_end.len();
                    let skip_end_index = padded.len() - 1 - skip_end;
                    padded
                        .windows(group + 2)
                        .enumerate()
                        .skip(skip_start)
                        .take_while(move |(index, _)| *index < skip_end_index)
                        .filter_map(move |(index, window)| {
                            let first = window[0];
                            let last = window[window.len() - 1];
                            ((first == State::Operational || first == State::Unknown)
                                && window[1..(window.len() - 1)]
                                    .into_iter()
                                    .all(|&s| s == State::Damaged || s == State::Unknown)
                                && (last == State::Operational || last == State::Unknown))
                                .then(move || (index, group))
                        })
                })
                .multi_cartesian_product()
                .filter(|placements| {
                    placements
                        .windows(2)
                        .all(|window| window[0].0 + window[0].1 < window[1].0)
                })
                .filter(|placements| {
                    damaged.iter().all(|index| {
                        placements
                            .binary_search_by(|&(i, len)| {
                                if (i..=(i + len)).contains(index) {
                                    Ordering::Equal
                                } else {
                                    i.cmp(index)
                                }
                            })
                            .is_ok()
                    })
                })
                .count();
            variants
        })
        .sum()
}
