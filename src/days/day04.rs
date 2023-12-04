use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, multispace1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{days::Day, helpers::parse_digit};

pub struct Day04;

#[derive(Clone, Debug)]
pub struct Card {
    id: usize,
    winning: Vec<usize>,
    selected: Vec<usize>,
}

impl Day for Day04 {
    type Input = Vec<Card>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        // let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        all_consuming(separated_list1(
            line_ending,
            map(
                tuple((
                    preceded(
                        preceded(multispace0, terminated(tag("Card"), multispace1)),
                        parse_digit("card id"),
                    ),
                    preceded(
                        terminated(tag(":"), multispace1),
                        separated_list1(multispace1, parse_digit("winning number")),
                    ),
                    preceded(
                        delimited(multispace1, tag("|"), multispace1),
                        separated_list1(multispace1, parse_digit("selected number")),
                    ),
                )),
                |(id, winning, selected)| Card {
                    id,
                    winning,
                    selected,
                },
            ),
        ))(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|card| {
                let mut points = 0;

                for n in card.selected.iter() {
                    if card.winning.contains(n) {
                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                    }
                }

                points
            })
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut cards = input.clone();
        let mut total = cards.len();

        while let Some(card) = cards.pop() {
            let matching = card
                .selected
                .iter()
                .filter(|n| card.winning.contains(n))
                .count();
            for id in (card.id)..(card.id + matching) {
                if let Some(card) = input.get(id) {
                    cards.push(card.clone());
                    total += 1;
                }
            }
        }

        total
    }
}
