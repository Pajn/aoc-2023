use std::{cmp::Ordering, collections::BTreeMap};

use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, line_ending, multispace0, multispace1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{days::Day, helpers::parse_digit};

const CARDS: [char; 14] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '*',
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Score {
    Five,
    Four,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord)]
pub struct Hand([char; 5], usize);

impl Hand {
    fn make_jokers(&mut self) {
        for card in self.0.iter_mut() {
            if *card == 'J' {
                *card = '*'
            }
        }
    }

    fn score(&self) -> Score {
        let mut cards = BTreeMap::new();
        for card in self.0 {
            cards.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }

        if cards.len() > 1 {
            if let Some(n) = cards.get(&'*').copied() {
                cards.remove(&'*');
                let highest = *cards
                    .iter()
                    .sorted_by_key(|(_, c)| *c)
                    .next_back()
                    .unwrap()
                    .0;
                cards.entry(highest).and_modify(|v| *v += n);
            }
        }

        let mut values = cards.values().sorted().collect_vec();
        match values.pop().unwrap() {
            5 => Score::Five,
            4 => Score::Four,
            3 => match values.pop().unwrap() {
                2 => Score::FullHouse,
                _ => Score::ThreeOfAKind,
            },
            2 => match values.pop().unwrap() {
                2 => Score::TwoPair,
                _ => Score::Pair,
            },
            _ => Score::High,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score().cmp(&other.score()).then_with(|| {
            self.0
                .iter()
                .zip_eq(other.0.iter())
                .fold(Ordering::Equal, |ord, (a, b)| {
                    ord.then_with(|| {
                        let a = CARDS.iter().position(|c| c == a).unwrap();
                        let b = CARDS.iter().position(|c| c == b).unwrap();

                        a.cmp(&b)
                    })
                })
        }))
    }
}

pub struct Day07;

impl Day for Day07 {
    type Input = Vec<Hand>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "32T3K 765
        // T55J5 684
        // KK677 28
        // KTJJT 220
        // QQQJA 483";

        all_consuming(separated_list1(
            preceded(line_ending, multispace0),
            map(
                tuple((alphanumeric1, preceded(multispace1, parse_digit("bid")))),
                |(cards, bid): (&str, usize)| {
                    Hand(
                        cards.chars().collect_vec().try_into().expect("5 cards"),
                        bid,
                    )
                },
            ),
        ))(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut input = input.clone();
        input.sort();

        let mut winnings = 0;

        for (i, hand) in input.iter().enumerate() {
            let rank = input.len() - i;
            winnings += hand.1 * rank;
        }

        winnings
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut input = input.clone();
        for hand in input.iter_mut() {
            hand.make_jokers();
        }
        input.sort();

        let mut winnings = 0;

        for (i, hand) in input.iter().enumerate() {
            let rank = input.len() - i;
            winnings += hand.1 * rank;
        }

        winnings
    }
}
