use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{days::Day, helpers::parse_digit};

pub struct Day02;

#[derive(Default, Debug)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Default, Debug)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Day for Day02 {
    type Input = Vec<Game>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        separated_list1(newline, parse_game)(input)
    }

    type Output1 = u32;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        // println!("{:?}", input);
        let red = 12;
        let green = 13;
        let blue = 14;
        let mut sum = 0;

        for game in input {
            let max_red = game.rounds.iter().max_by_key(|r| r.red).unwrap().red;
            let max_green = game.rounds.iter().max_by_key(|r| r.green).unwrap().green;
            let max_blue = game.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;

            if max_red <= red && max_green <= green && max_blue <= blue {
                sum += game.id;
            }
        }

        sum
    }

    type Output2 = u32;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut sum = 0;

        for game in input {
            let max_red = game.rounds.iter().max_by_key(|r| r.red).unwrap().red;
            let max_green = game.rounds.iter().max_by_key(|r| r.green).unwrap().green;
            let max_blue = game.rounds.iter().max_by_key(|r| r.blue).unwrap().blue;

            let power = max_red * max_green * max_blue;

            sum += power;
        }

        sum
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((
            preceded(preceded(multispace0, tag("Game ")), parse_digit("game id")),
            preceded(tag(": "), separated_list1(tag("; "), parse_round)),
        )),
        |(id, rounds)| Game { id, rounds },
    )(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    map(
        separated_list1(
            tag(", "),
            tuple((
                parse_digit("number of tiles"),
                preceded(tag(" "), alt((tag("red"), tag("green"), tag("blue")))),
            )),
        ),
        |colors| {
            colors
                .into_iter()
                .fold(Round::default(), |mut round, (tiles, color)| {
                    match color {
                        "red" => {
                            round.red = tiles;
                        }
                        "green" => {
                            round.green = tiles;
                        }
                        "blue" => {
                            round.blue = tiles;
                        }
                        _ => unreachable!("Unknown color {color}"),
                    };
                    round
                })
        },
    )(input)
}
