use std::fmt::Write;

use crate::map::{Element as Part1Element, Map as Part1Map};

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
                        Part1Element::Box => vec![Element::BoxOpens, Element::BoxCloses],
                        Part1Element::Robot => vec![Element::Robot, Element::Empty],
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Map(grid)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for row in self.0.iter() {
            for el in row.iter() {
                write!(f, "{el}")?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Element {
    Empty,
    Wall,
    BoxOpens,
    BoxCloses,
    Robot,
}

impl Element {
    fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Wall => '#',
            Element::BoxOpens => '[',
            Element::BoxCloses => ']',
            Element::Robot => '@',
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
            "\n####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################\n",
            Map::from(part1_map).to_string().as_str()
        );
    }
}
