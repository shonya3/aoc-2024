use crate::position::Position;
use std::{fmt::Write, str::FromStr};

pub const MAP_EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

pub const MAP_EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

#[derive(Debug, Clone)]
pub struct Map(pub Vec<Vec<Element>>);

impl Map {
    pub fn get(&self, position: Position) -> Option<Element> {
        self.0
            .get(position.y)
            .and_then(|row| row.get(position.x))
            .copied()
    }

    pub fn find_start_position(&self) -> Option<Position> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, el)| (x, y, el)))
            .find_map(|(x, y, el)| matches!(el, Element::Start).then_some(Position { x, y }))
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last_row_i = self.0.iter().len() - 1;
        for (i, row) in self.0.iter().enumerate() {
            for el in row.iter() {
                write!(f, "{el}")?;
            }

            if i != last_row_i {
                f.write_char('\n')?
            };
        }

        Ok(())
    }
}

impl FromStr for Map {
    type Err = ParseElementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| Element::from_str(&ch.to_string()))
                    .collect::<Result<Vec<Element>, ParseElementError>>()
            })
            .collect::<Result<Vec<Vec<Element>>, ParseElementError>>(
            )?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Empty,
    Wall,
    Start,
    End,
}

impl Element {
    fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Wall => '#',
            Element::Start => 'S',
            Element::End => 'E',
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[derive(Debug)]
pub struct ParseElementError(pub String);

impl FromStr for Element {
    type Err = ParseElementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Element::Empty),
            "#" => Ok(Element::Wall),
            "S" => Ok(Element::Start),
            "E" => Ok(Element::End),
            _ => Err(ParseElementError(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::map::MAP_EXAMPLE;

    #[test]
    fn parse() {
        let map: Map = MAP_EXAMPLE.parse().unwrap();
        println!("{map}");
    }
}
