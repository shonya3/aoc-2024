#![allow(unused)]

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub struct Garden(pub Vec<Vec<Plant>>);

impl Garden {
    pub fn fence_price(&self) -> usize {
        self.regions()
            .iter()
            .map(|region| region.fence_price())
            .sum()
    }

    pub fn get(&self, position: Position) -> Option<Plant> {
        self.0
            .get(position.y)
            .and_then(|row| row.get(position.x))
            .copied()
    }

    pub fn regions(&self) -> Vec<Region> {
        use direction::Direction;
        use std::collections::{HashSet, VecDeque};

        const DIRECTIONS: [Direction; 4] = Direction::directions();
        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, plant)| {
                let position = Position { x, y };
                if visited.contains(&position) {
                    return;
                }

                let mut queue = VecDeque::new();
                let mut positions = Vec::new();

                queue.push_back(position);
                visited.insert(position);

                while let Some(pos) = queue.pop_front() {
                    positions.push(pos);

                    for direction in DIRECTIONS {
                        if let Some(neighbor) = direction::next_position(pos, direction) {
                            if self.get(neighbor) == Some(*plant) && !visited.contains(&neighbor) {
                                visited.insert(neighbor);
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }

                regions.push(Region {
                    garden: self,
                    plant: *plant,
                    positions,
                });
            })
        });

        regions
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<Plant>> = Vec::new();

        Ok(Garden(
            s.lines()
                .map(|line: &str| line.chars().map(Plant).collect())
                .collect(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Region<'garden> {
    pub garden: &'garden Garden,
    pub plant: Plant,
    pub positions: Vec<Position>,
}

impl Region<'_> {
    pub fn fence_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    pub fn perimeter(&self) -> usize {
        use direction::{next_position, Direction};
        let directions = Direction::directions();
        let plant = self.plant;

        let garden = self.garden;
        self.positions
            .iter()
            .map(|position| {
                directions
                    .iter()
                    .filter(|direction| {
                        next_position(*position, **direction)
                            .and_then(|position| garden.get(position))
                            != Some(self.plant)
                    })
                    .collect::<Vec<_>>()
                    .len()
            })
            .sum()
    }

    pub fn area(&self) -> usize {
        self.positions.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Plant(pub char);

mod direction {
    use crate::garden::Position;

    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    pub enum Direction {
        Left,
        Up,
        Right,
        Down,
    }

    impl Direction {
        pub const fn directions() -> [Direction; 4] {
            [
                Direction::Up,
                Direction::Left,
                Direction::Right,
                Direction::Down,
            ]
        }
    }

    pub fn next_position(position: Position, direction: Direction) -> Option<Position> {
        let Position { x, y } = position;
        match direction {
            Direction::Left => match x == 0 {
                true => None,
                false => Some(Position { x: x - 1, y }),
            },
            Direction::Up => match y == 0 {
                true => None,
                false => Some(Position { x, y: y - 1 }),
            },
            Direction::Right => Some(Position { x: x + 1, y }),
            Direction::Down => Some(Position { x, y: y + 1 }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Garden, Region};
    use crate::garden::{Plant, Position};
    use std::collections::HashMap;

    const SIMPLE_EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    const LARGER_EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn from_str() {
        let garden: Garden = SIMPLE_EXAMPLE.parse().unwrap();
        assert_eq!(Some(Plant('E')), garden.get(Position { x: 0, y: 3 }));
        assert_eq!(Some(Plant('D')), garden.get(Position { x: 3, y: 1 }));
    }

    #[test]
    fn price() {
        assert_eq!(140, SIMPLE_EXAMPLE.parse::<Garden>().unwrap().fence_price());
        assert_eq!(
            1930,
            LARGER_EXAMPLE.parse::<Garden>().unwrap().fence_price()
        );
    }
}
