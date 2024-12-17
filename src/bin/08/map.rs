use std::{fmt::Write, str::FromStr};

#[derive(Debug, Clone)]
pub struct Map(pub Vec<Vec<Element>>);

#[allow(unused)]
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
                let _ = f.write_char(element.as_char());
            }
            if i != latest_row {
                let _ = f.write_str("\n");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Empty,
    Antinode,
    Antenna(AntennaId),
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct AntennaId(pub char);

impl Element {
    pub fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Antinode => '#',
            Element::Antenna(id) => id.0,
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct ParseElementError(String);

impl FromStr for Element {
    type Err = ParseElementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Element::Empty),
            "#" => Ok(Element::Antinode),
            s => {
                let Some(ch) = s.chars().next() else {
                    return Err(ParseElementError(s.to_owned()));
                };

                if s.len() != 1 {
                    return Err(ParseElementError(s.to_owned()));
                }

                Ok(Element::Antenna(AntennaId(ch)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map::{AntennaId, Element};

    use super::Map;

    const EXAMPLE_DATA: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn parse_map_from_str() {
        let map: Map = EXAMPLE_DATA.parse().unwrap();
        assert_eq!(
            map.0[4],
            vec![
                Element::Empty,
                Element::Empty,
                Element::Empty,
                Element::Empty,
                Element::Antenna(AntennaId('0')),
                Element::Empty,
                Element::Empty,
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
        assert_eq!(EXAMPLE_DATA, map.to_string());
    }
}
