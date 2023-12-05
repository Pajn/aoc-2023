use std::{cmp::Ordering, ops::Range, slice::Iter};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, multispace1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{days::Day, helpers::parse_digit};

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(Range<usize>, usize)>,
    soil_to_fertilizer: Vec<(Range<usize>, usize)>,
    fertilizer_to_water: Vec<(Range<usize>, usize)>,
    water_to_light: Vec<(Range<usize>, usize)>,
    light_to_temperature: Vec<(Range<usize>, usize)>,
    temperature_to_humidity: Vec<(Range<usize>, usize)>,
    humidity_to_location: Vec<(Range<usize>, usize)>,
}

impl Almanac {
    fn lookup(&self, key: usize, map: &[(Range<usize>, usize)]) -> usize {
        map.binary_search_by(|(src, _dst)| {
            if src.contains(&key) {
                Ordering::Equal
            } else {
                src.start.cmp(&key)
            }
        })
        .map_or(key, |i| {
            let (ref src, dst) = map[i];
            dst + key - src.start
        })
    }

    fn multi_lookup(&self, key: usize, maps: &[&[(Range<usize>, usize)]]) -> usize {
        maps.iter().fold(key, |key, map| self.lookup(key, map))
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        self.multi_lookup(
            seed,
            &[
                &self.seed_to_soil,
                &self.soil_to_fertilizer,
                &self.fertilizer_to_water,
                &self.water_to_light,
                &self.light_to_temperature,
                &self.temperature_to_humidity,
                &self.humidity_to_location,
            ],
        )
    }

    fn seeds(&self) -> SeedsIter {
        SeedsIter {
            seeds: self.seeds.iter(),
            range: None,
        }
    }
}

struct SeedsIter<'a> {
    seeds: Iter<'a, usize>,
    range: Option<Range<usize>>,
}

impl<'a> Iterator for SeedsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.range.as_mut().and_then(|range| range.next());

        if next.is_some() {
            return next;
        };

        if let (Some(start), Some(len)) = (self.seeds.next(), self.seeds.next()) {
            self.range = Some(*start..(start + len))
        }

        self.range.as_mut().and_then(|range| range.next())
    }
}

pub struct Day05;

impl Day for Day05 {
    type Input = Almanac;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        // let input = "seeds: 79 14 55 13

        // seed-to-soil map:
        // 50 98 2
        // 52 50 48

        // soil-to-fertilizer map:
        // 0 15 37
        // 37 52 2
        // 39 0 15

        // fertilizer-to-water map:
        // 49 53 8
        // 0 11 42
        // 42 0 7
        // 57 7 4

        // water-to-light map:
        // 88 18 7
        // 18 25 70

        // light-to-temperature map:
        // 45 77 23
        // 81 45 19
        // 68 64 13

        // temperature-to-humidity map:
        // 0 69 1
        // 1 0 69

        // humidity-to-location map:
        // 60 56 37
        // 56 93 4";

        map(
            all_consuming(tuple((
                preceded(
                    delimited(multispace0, tag("seeds:"), multispace1),
                    separated_list1(multispace1, parse_digit("seed")),
                ),
                preceded(
                    delimited(multispace0, tag("seed-to-soil map:"), multispace1),
                    parse_map,
                ),
                preceded(
                    delimited(multispace0, tag("soil-to-fertilizer map:"), multispace1),
                    parse_map,
                ),
                preceded(
                    delimited(multispace0, tag("fertilizer-to-water map:"), multispace1),
                    parse_map,
                ),
                preceded(
                    delimited(multispace0, tag("water-to-light map:"), multispace1),
                    parse_map,
                ),
                preceded(
                    delimited(multispace0, tag("light-to-temperature map:"), multispace1),
                    parse_map,
                ),
                preceded(
                    delimited(
                        multispace0,
                        tag("temperature-to-humidity map:"),
                        multispace1,
                    ),
                    parse_map,
                ),
                preceded(
                    delimited(multispace0, tag("humidity-to-location map:"), multispace1),
                    parse_map,
                ),
            ))),
            |(
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            )| Almanac {
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            },
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .seeds
            .iter()
            .map(|seed| input.seed_to_location(*seed))
            .min()
            .expect("location")
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .seeds()
            .map(|seed| input.seed_to_location(seed))
            .min()
            .expect("location")
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<(Range<usize>, usize)>> {
    map(
        separated_list1(
            line_ending,
            tuple((
                preceded(multispace0, parse_digit("dst start")),
                preceded(multispace1, parse_digit("src start")),
                preceded(multispace1, parse_digit("range length")),
            )),
        ),
        |mut ranges: Vec<(usize, usize, usize)>| {
            let mut map = vec![];

            ranges.sort_by_key(|(_dst_start, src_start, _length)| *src_start);

            for (dst_start, src_start, length) in ranges {
                map.push((src_start..(src_start + length), dst_start));
            }

            map
        },
    )(input)
}
