use crate::position::Position;
use std::str::FromStr;

pub const ROTATIONS: [Rotation; 2] = Rotation::rotations();

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn as_char(&self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl Rotation {
    pub const fn rotations() -> [Rotation; 2] {
        [Rotation::Clockwise, Rotation::Counterclockwise]
    }
}

pub fn rotate_90deg(direction: Direction, rotation: Rotation) -> Direction {
    match direction {
        Direction::Left => match rotation {
            Rotation::Clockwise => Direction::Up,
            Rotation::Counterclockwise => Direction::Down,
        },
        Direction::Up => match rotation {
            Rotation::Clockwise => Direction::Right,
            Rotation::Counterclockwise => Direction::Left,
        },
        Direction::Right => match rotation {
            Rotation::Clockwise => Direction::Down,
            Rotation::Counterclockwise => Direction::Up,
        },
        Direction::Down => match rotation {
            Rotation::Clockwise => Direction::Left,
            Rotation::Counterclockwise => Direction::Right,
        },
    }
}
