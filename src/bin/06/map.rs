use std::{f32::consts::E, fmt::Write, path::Display, str::FromStr};

use crate::guard::Direction;

#[derive(Debug)]
pub struct Map(pub Vec<Vec<Element>>);

#[derive(Debug)]
pub struct ParseMapError(pub ParseElementError);

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = s
            .lines()
            .map(|line| {
                let result = line
                    .chars()
                    .map(|c| {
                        let s = String::from(c);
                        s.parse::<Element>()
                    })
                    .collect::<Result<Vec<Element>, ParseElementError>>();
                result
            })
            .collect::<Result<Vec<Vec<Element>>, ParseElementError>>()
            .map_err(ParseMapError)?;

        Ok(Map(matrix))
    }
}

impl Map {
    pub fn get(&self, i: usize, j: usize) -> Option<Element> {
        self.0.get(i).and_then(|row| row.get(j)).copied()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let latest_row = self.0.len() - 1;
        for (i, row) in self.0.iter().enumerate() {
            for element in row.iter() {
                f.write_char(element.as_char());
            }
            if i != latest_row {
                f.write_str("\n");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct GuardElement {
    pub direction: Direction,
}

#[derive(Debug)]
pub struct GuardElementParseError(pub String);
impl FromStr for GuardElement {
    type Err = GuardElementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "^" => Direction::Up,
            ">" => Direction::Right,
            "v" => Direction::Down,
            "<" => Direction::Left,
            _ => return Err(GuardElementParseError(s.to_owned())),
        };

        Ok(GuardElement { direction })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Empty,
    Obstacle,
    Guard(GuardElement),
    Visited,
}

impl Element {
    pub fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Obstacle => '#',
            Element::Guard(guard_element) => match guard_element.direction {
                Direction::Left => '<',
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
            },
            Element::Visited => 'X',
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[derive(Debug)]
pub struct ParseElementError(String);

impl FromStr for Element {
    type Err = ParseElementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Element::Empty),
            "#" => Ok(Element::Obstacle),
            _ => match GuardElement::from_str(s) {
                Ok(guard) => Ok(Element::Guard(guard)),
                Err(_) => Err(ParseElementError(s.to_owned())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{guard::Direction, map::Element};

    use super::{GuardElement, Map};

    const EXAMPLE_DATA: &str = r"#....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...#";

    #[test]
    fn parse_map_from_str() {
        let map: Map = EXAMPLE_DATA.parse().unwrap();
        assert_eq!(
            map.0[6],
            vec![
                Element::Empty,
                Element::Obstacle,
                Element::Empty,
                Element::Empty,
                Element::Guard(GuardElement {
                    direction: Direction::Up
                }),
                Element::Empty,
                Element::Empty,
                Element::Empty,
                Element::Empty,
                Element::Empty,
            ]
        )
    }

    #[test]
    fn print_map() {
        let map: Map = EXAMPLE_DATA.parse().unwrap();
        println!("{map}");
        assert_eq!(EXAMPLE_DATA, map.to_string());
    }
}
