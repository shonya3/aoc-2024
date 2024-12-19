use crate::map::Map;

#[derive(Debug, Clone)]
pub struct Walker<'map> {
    pub position: Position,
    pub map: &'map Map,
    pub path: Path,
    pub status: PathStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum PathStatus {
    Done,
    Target(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub start: Position,
    pub directions: Vec<Direction>,
}

#[derive(Debug, Clone, Copy)]
pub enum WalkError {
    AlreadyDone,
    IncorrectTarget {
        expected: u8,
        actual: u8,
    },
    NextPositionNotExists {
        position: Position,
        direction: Direction,
    },
    NoValue(Position),
    ValueCannotBeMoreThanNine,
}

impl Walker<'_> {
    pub fn step(&mut self, direction: Direction) -> Result<(), WalkError> {
        self.path.directions.push(direction);
        self.position =
            next_position(self.position, direction).ok_or(WalkError::NextPositionNotExists {
                position: self.position,
                direction,
            })?;

        let value = self
            .map
            .get(self.position.i, self.position.j)
            .ok_or(WalkError::NoValue(self.position))?;

        match self.status {
            PathStatus::Done => Err(WalkError::AlreadyDone),
            PathStatus::Target(target) => {
                if value != target {
                    return Err(WalkError::IncorrectTarget {
                        expected: target,
                        actual: value,
                    });
                }

                match value {
                    9 => {
                        self.status = PathStatus::Done;
                    }
                    value if value > 9 => return Err(WalkError::ValueCannotBeMoreThanNine),
                    _ => self.status = PathStatus::Target(value + 1),
                };

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

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

fn next_position(position: Position, direction: Direction) -> Option<Position> {
    let Position { i, j } = position;
    let (i, j) = match direction {
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
    }?;

    Some(Position { i, j })
}
