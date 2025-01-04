use crate::{direction::Direction, map::Map, position::Position};

#[derive(Debug)]
pub struct Solution<'map> {
    pub position: Position,
    pub map: &'map mut Map,
    pub end: Position,
    pub start: Position,
    pub moves: Vec<Move>,
    pub direction: Direction,
}

impl Solution<'_> {
    pub fn make_a_move(&mut self) {
        let mut queue: Vec<Solution> = Vec::new();
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Step(Direction),
    Rotate90Degree(Rotation),
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}
