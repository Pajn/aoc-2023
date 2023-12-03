use std::collections::BTreeMap;

use nom::{
    branch::alt,
    character::complete::{anychar, char, line_ending},
    combinator::map,
    multi::many1,
    IResult,
};

use crate::{days::Day, helpers::parse_digit};

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Dot,
    Newline,
    Char(char),
}

impl Token {
    fn len(&self) -> i32 {
        match self {
            Token::Number(n) => (*n as f32).log10().floor() as i32 + 1,
            _ => 1,
        }
    }
}

pub struct Day03;

impl Day for Day03 {
    type Input = BTreeMap<(i32, i32), (char, Vec<((i32, i32), u32)>)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..";

        map(
            many1(alt((
                map(parse_digit("number"), Token::Number),
                map(char('.'), |_| Token::Dot),
                map(line_ending, |_| Token::Newline),
                map(anychar, Token::Char),
            ))),
            |tokens| {
                let mut numbers = Vec::new();
                let mut parts = BTreeMap::new();

                let mut row = 0;
                let mut col = 0;
                for token in tokens {
                    // println!("({row}:{col}): {token:?}");

                    match token {
                        Token::Number(n) => {
                            numbers.push((row, col, col + token.len(), n));
                        }
                        Token::Char(c) => {
                            parts.insert((row, col), (c, Vec::new()));
                        }
                        _ => {}
                    };

                    if token == Token::Newline {
                        row += 1;
                        col = 0;
                    } else {
                        col += token.len();
                    }
                }

                for (num_row, start_col, end_col, n) in numbers.iter() {
                    let x = (start_col - 1)..=(*end_col);
                    let y = (num_row - 1)..=(num_row + 1);

                    for ((row, col), (_part, nums)) in parts.iter_mut() {
                        // println!("x: {x:?} - {col}");
                        // println!("y: {y:?} - {row}");

                        if x.contains(col) && y.contains(row) {
                            nums.push(((*num_row, *start_col), *n));
                        }
                    }
                }

                // println!("parts {parts:#?}");

                parts
            },
        )(input)
    }

    type Output1 = u32;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .values()
            .flat_map(|(_, nums)| nums)
            .copied()
            .collect::<BTreeMap<(i32, i32), u32>>()
            .values()
            .sum()
        // 4361
    }

    type Output2 = u32;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .values()
            .filter_map(|(part, nums)| -> Option<u32> {
                if *part == '*' && nums.len() == 2 {
                    Some(nums.iter().map(|(_, n)| n).product())
                } else {
                    None
                }
            })
            .sum()
        //467835
    }
}
