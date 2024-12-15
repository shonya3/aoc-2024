use crate::map::{Element, Map};
use std::str::FromStr;

#[derive(Debug)]
pub struct Guard<'map> {
    pub position: Position,
    pub map: &'map mut Map,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub enum Position {
    Map(usize, usize),
    OutOfMap,
}

#[derive(Debug)]
pub enum StepError {
    UnexpectedAnotherGuard(usize, usize),
    GuardIsAlreadyOutOfMap,
}

impl Guard<'_> {
    pub fn step(&mut self) -> Result<(), StepError> {
        match self.position {
            Position::Map(current_i, current_j) => {
                let Some((next_i, next_j)) = next_i_j(current_i, current_j, self.direction) else {
                    self.map.0[current_i][current_j] = Element::Visited;
                    self.position = Position::OutOfMap;
                    return Ok(());
                };

                let Some(element) = self.map.get(next_i, next_j) else {
                    self.map.0[current_i][current_j] = Element::Visited;
                    self.position = Position::OutOfMap;
                    return Ok(());
                };

                match element {
                    Element::Empty | Element::Visited => {
                        self.map.0[current_i][current_j] = Element::Visited;
                        self.position = Position::Map(next_i, next_j);
                    }
                    Element::Obstacle => {
                        self.direction = self.direction.rotate_90deg();
                    }
                    Element::Guard(guard_element) => {
                        return Err(StepError::UnexpectedAnotherGuard(next_i, next_j))
                    }
                }

                Ok(())
            }
            Position::OutOfMap => Err(StepError::GuardIsAlreadyOutOfMap),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn rotate_90deg(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

fn next_i_j(i: usize, j: usize, direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Left => match j == 0 {
            true => None,
            false => Some((i, j - 1)),
        },
        Direction::Up => match i == 0 {
            true => None,
            false => Some((i - 1, j)),
        },
        Direction::Right => Some((i, j + 1)),
        Direction::Down => Some((i + 1, j)),
    }
}
