use std::cell::RefCell;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace0, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use num::integer::lcm;

use crate::days::Day;

#[derive(Debug)]
pub struct Data {
    steps: Vec<char>,
    names: Vec<String>,
    nodes: Vec<(usize, usize)>,
}

pub struct Day08;

impl Day for Day08 {
    type Input = Data;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "LLR

        // AAA = (BBB, BBB)
        // BBB = (AAA, ZZZ)
        // ZZZ = (ZZZ, ZZZ)";

        let names = RefCell::new(vec![]);
        fn parse_node<'a>(names: &'a RefCell<Vec<String>>) -> impl FnMut(&str) -> usize + 'a {
            |node: &str| {
                let mut names = names.borrow_mut();
                if let Some(index) = names.iter().position(|n| n == node) {
                    index
                } else {
                    names.push(node.into());
                    names.len() - 1
                }
            }
        }

        let (input, (steps, mut nodes)) = tuple((
            terminated(many1(one_of("RL")), many1(line_ending)),
            separated_list1(
                preceded(line_ending, multispace0),
                tuple((
                    preceded(multispace0, map(alphanumeric1, parse_node(&names))),
                    preceded(
                        tag(" = ("),
                        tuple((
                            map(alphanumeric1, parse_node(&names)),
                            delimited(tag(", "), map(alphanumeric1, parse_node(&names)), tag(")")),
                        )),
                    ),
                )),
            ),
        ))(input)?;
        nodes.sort_by_key(|n| n.0);

        Ok((
            input,
            Data {
                steps,
                names: names.take(),
                nodes: nodes.into_iter().map(|n| n.1).collect(),
            },
        ))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let start = input.names.iter().position(|n| n == "AAA").expect("start");
        let stop = input.names.iter().position(|n| n == "ZZZ").expect("stop");

        let steps = input.steps.iter().cycle();
        let mut current = start;
        let mut count = 0;
        for step in steps {
            match step {
                'L' => current = input.nodes[current].0,
                'R' => current = input.nodes[current].1,
                _ => unreachable!(),
            }
            count += 1;
            if current == stop {
                break;
            }
        }

        count
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let starts = input
            .names
            .iter()
            .positions(|n| n.ends_with("A"))
            .collect_vec();

        let start_counts = starts.into_iter().map(|start| {
            let steps = input.steps.iter().cycle();
            let mut current = start;
            let mut count = 0;
            for step in steps {
                match step {
                    'L' => current = input.nodes[current].0,
                    'R' => current = input.nodes[current].1,
                    _ => unreachable!(),
                }
                count += 1;

                if input.names[current].ends_with("Z") {
                    break;
                }
            }
            count
        });

        start_counts.reduce(lcm).unwrap()
    }
}
