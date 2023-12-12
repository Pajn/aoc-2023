use std::{collections::BTreeSet, fmt::Display};

use array2d::Array2D;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, multispace1},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    IResult,
};

use crate::days::Day;
pub struct Map(Array2D<bool>);

impl Map {
    fn galaxies<'a>(&'a self) -> impl Iterator<Item = (isize, isize)> + 'a {
        let cols = self.0.num_columns();
        self.0
            .elements_row_major_iter()
            .enumerate()
            .filter_map(move |(index, &value)| {
                if value {
                    let row = index / cols;
                    let col = index % cols;
                    Some((row as isize, col as isize))
                } else {
                    None
                }
            })
    }

    fn total_travel_distance(&self, expansion_rate: usize) -> usize {
        let added_cost = expansion_rate - 1;

        let mut rows_to_duplicate = BTreeSet::new();
        for (row, mut values) in self.0.rows_iter().enumerate() {
            if values.all(|v| !v) {
                rows_to_duplicate.insert(row as isize);
            }
        }

        let mut cols_to_duplicate = BTreeSet::new();
        for (col, mut values) in self.0.columns_iter().enumerate() {
            if values.all(|v| !v) {
                cols_to_duplicate.insert(col as isize);
            }
        }

        self.galaxies()
            .combinations(2)
            .map(|pair| {
                let a = pair[0];
                let b = pair[1];

                let rows = (a.0.min(b.0))..=(a.0.max(b.0));
                let cols = (a.1.min(b.1))..=(a.1.max(b.1));

                let path = rows.zip_longest(cols).map(|pair| {
                    use itertools::EitherOrBoth as Zip;
                    match pair {
                        Zip::Both(row, col) => (row, col),
                        Zip::Left(row) => (row, b.1),
                        Zip::Right(col) => (b.0, col),
                    }
                });

                let mut cost = ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize;
                for (row, col) in path {
                    if rows_to_duplicate.contains(&row) {
                        cost += added_cost
                    }
                    if cols_to_duplicate.contains(&col) {
                        cost += added_cost
                    }
                }
                cost
            })
            .sum()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.rows_iter() {
            for value in row {
                if *value {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Day11;

impl Day for Day11 {
    type Input = Map;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "...#......
        // .......#..
        // #.........
        // ..........
        // ......#...
        // .#........
        // .........#
        // ..........
        // .......#..
        // #...#.....";

        map(
            all_consuming(separated_list1(
                multispace1,
                many1(alt((value(true, char('#')), value(false, char('.'))))),
            )),
            |map| Map(Array2D::from_rows(&map).expect("Well formed map")),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.total_travel_distance(2)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.total_travel_distance(1_000_000)
    }
}
