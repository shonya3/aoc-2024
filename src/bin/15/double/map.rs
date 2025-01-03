use std::{fmt::Write, str::FromStr};

use crate::{
    map::{Element as Part1Element, Map as Part1Map},
    position::Position,
};

impl Map {
    pub fn get(&self, position: Position) -> Option<Element> {
        self.0
            .get(position.y)
            .and_then(|row| row.get(position.x))
            .copied()
    }

    pub fn find_robot_position(&self) -> Option<Position> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, el)| (x, y, el)))
            .find_map(|(x, y, el)| matches!(el, Element::Robot).then_some(Position { x, y }))
    }

    pub fn boxes_gps(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, el)| {
                    matches!(el, Element::Box(BoxEl::Opening)).then_some(Position { x, y })
                })
            })
            .map(|position| 100 * position.y + position.x)
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Map(pub Vec<Vec<Element>>);

impl From<Part1Map> for Map {
    fn from(part1_map: Part1Map) -> Self {
        let grid = part1_map
            .0
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|el| match el {
                        Part1Element::Empty => vec![Element::Empty, Element::Empty],
                        Part1Element::Wall => vec![Element::Wall, Element::Wall],
                        Part1Element::Box => {
                            vec![Element::Box(BoxEl::Opening), Element::Box(BoxEl::Closing)]
                        }
                        Part1Element::Robot => vec![Element::Robot, Element::Empty],
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Map(grid)
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

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last_row_i = self.0.len() - 1;
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

#[derive(Debug, Clone, Copy)]
pub enum Element {
    Empty,
    Wall,
    Box(BoxEl),
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoxEl {
    Opening,
    Closing,
}

impl Element {
    fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Wall => '#',
            Element::Robot => '@',
            Element::Box(box_el) => match box_el {
                BoxEl::Opening => '[',
                BoxEl::Closing => ']',
            },
        }
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
            "[" => Ok(Element::Box(BoxEl::Opening)),
            "]" => Ok(Element::Box(BoxEl::Closing)),
            "@" => Ok(Element::Robot),
            _ => Err(ParseElementError(s.to_owned())),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[cfg(test)]
mod tests {
    use crate::map::Map as Part1Map;

    use super::Map;

    #[test]
    fn from_part1_map() {
        let part1_map: Part1Map = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########"
            .parse()
            .unwrap();

        assert_eq!(
            "####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################",
            Map::from(part1_map).to_string().as_str()
        );
    }
}
