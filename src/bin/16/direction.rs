use std::str::FromStr;

use crate::position::Position;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
pub struct ParseDirectionError(pub String);
impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Direction::Left),
            "^" => Ok(Direction::Up),
            ">" => Ok(Direction::Right),
            "v" => Ok(Direction::Down),
            _ => Err(ParseDirectionError(s.to_owned())),
        }
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
