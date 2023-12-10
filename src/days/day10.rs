use std::{collections::BTreeSet, fmt::Display};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, multispace1},
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

use crate::days::Day;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
    Outside,
}

impl Tile {
    fn connects_north(self) -> bool {
        match self {
            Tile::Vertical | Tile::NorthWest | Tile::NorthEast => true,
            _ => false,
        }
    }

    fn connects_east(self) -> bool {
        match self {
            Tile::Horizontal | Tile::NorthEast | Tile::SouthEast => true,
            _ => false,
        }
    }

    fn connects_west(self) -> bool {
        match self {
            Tile::Horizontal | Tile::NorthWest | Tile::SouthWest => true,
            _ => false,
        }
    }

    fn connects_south(self) -> bool {
        match self {
            Tile::Vertical | Tile::SouthWest | Tile::SouthEast => true,
            _ => false,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Vertical => '│',
                Tile::Horizontal => '─',
                Tile::NorthEast => '└',
                Tile::NorthWest => '┘',
                Tile::SouthWest => '┐',
                Tile::SouthEast => '┌',
                Tile::Ground => ' ',
                Tile::Start => 'S',
                Tile::Outside => 'O',
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn tile(&self, (row, col): (usize, usize)) -> Tile {
        self.tiles[row][col]
    }
    fn north(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if row == 0 {
            None
        } else {
            Some((row - 1, col))
        }
    }
    fn east(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if col == self.tiles[0].len() - 1 {
            None
        } else {
            Some((row, col + 1))
        }
    }
    fn west(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if col == 0 {
            None
        } else {
            Some((row, col - 1))
        }
    }
    fn south(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        if row == self.tiles.len() - 1 {
            None
        } else {
            Some((row + 1, col))
        }
    }

    fn replace_start(&mut self) {
        let mut replacement = Tile::Start;

        for (row, row_tiles) in self.tiles.iter().enumerate() {
            for (col, tile) in row_tiles.iter().enumerate() {
                if *tile == Tile::Start {
                    self.start = (row, col);
                    let north = self
                        .north((row, col))
                        .map_or(false, |pos| self.tile(pos).connects_south());
                    let east = self
                        .east((row, col))
                        .map_or(false, |pos| self.tile(pos).connects_west());
                    let west = self
                        .west((row, col))
                        .map_or(false, |pos| self.tile(pos).connects_east());
                    let south = self
                        .south((row, col))
                        .map_or(false, |pos| self.tile(pos).connects_north());
                    replacement = match (north, east, south, west) {
                        (true, false, true, false) => Tile::Vertical,
                        (false, true, false, true) => Tile::Horizontal,
                        (true, true, false, false) => Tile::NorthEast,
                        (true, false, false, true) => Tile::NorthWest,
                        (false, true, true, false) => Tile::SouthEast,
                        (false, false, true, true) => Tile::SouthWest,
                        _ => panic!(
                            "Unexpected starting position: {:?}",
                            (north, east, south, west)
                        ),
                    };
                    break;
                }
            }
        }

        self.tiles[self.start.0][self.start.1] = replacement;
    }

    fn path<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (1..).scan((self.start, self.start), |(pos, prev), _| {
            let north = self.north(*pos);
            let south = self.south(*pos);
            let east = self.east(*pos);
            let west = self.west(*pos);

            let next = if north.is_some()
                && Some(*prev) != north
                && self.tile(*pos).connects_north()
                && self.tile(north.unwrap()).connects_south()
            {
                north
            } else if south.is_some()
                && Some(*prev) != south
                && self.tile(*pos).connects_south()
                && self.tile(south.unwrap()).connects_north()
            {
                south
            } else if east.is_some()
                && Some(*prev) != east
                && self.tile(*pos).connects_east()
                && self.tile(east.unwrap()).connects_west()
            {
                east
            } else if west.is_some()
                && Some(*prev) != west
                && self.tile(*pos).connects_west()
                && self.tile(west.unwrap()).connects_east()
            {
                west
            } else {
                panic!("No path from {:?}->{:?}", prev, pos);
            };

            *prev = *pos;
            *pos = next.unwrap();
            Some(*prev)
        })
    }

    fn cleanup(&mut self) {
        let mut path = BTreeSet::new();
        for pos in self.path() {
            if !path.insert(pos) {
                break;
            }
        }
        for (row, row_tiles) in self.tiles.iter_mut().enumerate() {
            for (col, tile) in row_tiles.iter_mut().enumerate() {
                if !path.contains(&(row, col)) {
                    *tile = Tile::Ground;
                }
            }
        }
    }

    fn scale_up(&mut self) {
        let scale = 3;
        let mut tiles =
            vec![vec![Tile::Ground; self.tiles[0].len() * scale]; self.tiles.len() * scale];
        for (row, tiles_row) in self.tiles.iter().enumerate() {
            for (col, tile) in tiles_row.iter().enumerate() {
                let scaled_tile = match tile {
                    Tile::Vertical => [
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                    ],
                    Tile::Horizontal => [
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                        [Tile::Horizontal, Tile::Horizontal, Tile::Horizontal],
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                    ],
                    Tile::NorthEast => [
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                        [Tile::Ground, Tile::NorthEast, Tile::Horizontal],
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                    ],
                    Tile::NorthWest => [
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                        [Tile::Horizontal, Tile::NorthWest, Tile::Ground],
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                    ],
                    Tile::SouthWest => [
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                        [Tile::Horizontal, Tile::SouthWest, Tile::Ground],
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                    ],
                    Tile::SouthEast => [
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                        [Tile::Ground, Tile::SouthEast, Tile::Horizontal],
                        [Tile::Ground, Tile::Vertical, Tile::Ground],
                    ],
                    Tile::Ground => [
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                        [Tile::Ground, Tile::Ground, Tile::Ground],
                    ],
                    Tile::Start => unimplemented!(),
                    Tile::Outside => [
                        [Tile::Outside, Tile::Outside, Tile::Outside],
                        [Tile::Outside, Tile::Outside, Tile::Outside],
                        [Tile::Outside, Tile::Outside, Tile::Outside],
                    ],
                };
                tiles[row * scale][col * scale] = scaled_tile[0][0];
                tiles[row * scale + 1][col * scale] = scaled_tile[1][0];
                tiles[row * scale + 2][col * scale] = scaled_tile[2][0];
                tiles[row * scale][col * scale + 1] = scaled_tile[0][1];
                tiles[row * scale + 1][col * scale + 1] = scaled_tile[1][1];
                tiles[row * scale + 2][col * scale + 1] = scaled_tile[2][1];
                tiles[row * scale][col * scale + 2] = scaled_tile[0][2];
                tiles[row * scale + 1][col * scale + 2] = scaled_tile[1][2];
                tiles[row * scale + 2][col * scale + 2] = scaled_tile[2][2];
            }
        }

        self.tiles = tiles;
    }

    fn scale_down(&mut self) {
        let scale = 3;
        let mut tiles =
            vec![vec![Tile::Ground; self.tiles[0].len() / scale]; self.tiles.len() / scale];
        for (row, tiles_row) in self.tiles.iter().enumerate().skip(scale / 2).step_by(scale) {
            for (col, center_tile) in tiles_row.iter().enumerate().skip(scale / 2).step_by(scale) {
                tiles[row / scale][col / scale] = *center_tile;
            }
        }

        self.tiles = tiles;
    }

    fn fill_outside(&mut self) {
        let row_range = 0..self.tiles.len();
        let col_range = 0..self.tiles[0].len();
        let top_edge = col_range.clone().map(|col| (0, col));
        let left_edge = row_range.clone().map(|row| (row, 0));
        let right_edge = row_range.clone().map(|row| (row, col_range.end - 1));
        let bottom_edge = col_range.clone().map(|col| (row_range.end - 1, col));
        let starts = top_edge
            .chain(left_edge)
            .chain(right_edge)
            .chain(bottom_edge)
            .filter(|pos| self.tile(*pos) == Tile::Ground)
            .collect_vec();

        let mut queue = starts;
        loop {
            let pos = match queue.pop() {
                Some(pos) => pos,
                None => break,
            };
            self.tiles[pos.0][pos.1] = Tile::Outside;

            let neighbors = [
                self.north(pos),
                self.south(pos),
                self.east(pos),
                self.west(pos),
            ]
            .into_iter()
            .filter_map(|pos| pos);

            for n in neighbors {
                let tile = self.tile(n);
                if tile == Tile::Ground {
                    queue.push(n);
                }
            }
        }
    }
}

pub struct Day10;

impl Day for Day10 {
    type Input = Map;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (input, tiles) = separated_list1(
            multispace1,
            many1(alt((
                value(Tile::Vertical, char('|')),
                value(Tile::Horizontal, char('-')),
                value(Tile::NorthEast, char('L')),
                value(Tile::NorthWest, char('J')),
                value(Tile::SouthWest, char('7')),
                value(Tile::SouthEast, char('F')),
                value(Tile::Ground, char('.')),
                value(Tile::Start, char('S')),
            ))),
        )(input)?;

        Ok((
            input,
            Map {
                tiles,
                start: (0, 0),
            },
        ))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut input = input.clone();
        input.replace_start();

        for (i, pos) in input.path().enumerate() {
            if i > 0 && pos == input.start {
                return i / 2;
            }
        }

        unreachable!();
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut input = input.clone();
        input.replace_start();
        input.cleanup();
        input.scale_up();
        input.fill_outside();
        input.scale_down();

        input
            .tiles
            .iter()
            .flat_map(|row| row)
            .filter(|tile| **tile == Tile::Ground)
            .count()
    }
}
