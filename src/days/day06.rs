use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::days::Day;

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn distance(&self, button_time: usize) -> usize {
        (self.time - button_time) * button_time
    }

    fn winning_moves<'a>(&'a self) -> impl ParallelIterator<Item = (usize, usize)> + 'a {
        (1..self.time)
            .into_par_iter()
            .map(|button_time| (button_time, self.distance(button_time)))
            .filter(|(_time, distance)| *distance > self.distance)
    }
}

pub struct Day06;

impl Day for Day06 {
    type Input = (Vec<Race>, Race);

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "Time:      7  15   30
        // Distance:  9  40  200";

        map(
            tuple((
                preceded(
                    delimited(multispace0, tag("Time:"), multispace1),
                    separated_list1(multispace1, digit1),
                ),
                preceded(
                    delimited(multispace0, tag("Distance:"), multispace1),
                    separated_list1(multispace1, digit1),
                ),
            )),
            |(times, distances)| {
                let races = times
                    .iter()
                    .zip_eq(distances.iter())
                    .map(|(time, distance): (&&str, &&str)| Race {
                        time: time.parse::<usize>().expect("time"),
                        distance: distance.parse::<usize>().expect("distance"),
                    })
                    .collect();
                let time = times.join("").parse().expect("time");
                let distance = distances.join("").parse().expect("distance");
                (races, Race { time, distance })
            },
        )(&input)
    }

    type Output1 = usize;

    fn part_1((races, _): &Self::Input) -> Self::Output1 {
        races
            .iter()
            .map(|race| race.winning_moves().count())
            .product()
    }

    type Output2 = usize;

    fn part_2((_, race): &Self::Input) -> Self::Output2 {
        race.winning_moves().count()
    }
}
