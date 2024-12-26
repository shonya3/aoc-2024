use crate::{direction::Direction, map::Map, position::Position};

#[derive(Debug)]
pub struct Robot<'map> {
    pub position: Position,
    pub map: &'map mut Map,
}

impl Robot<'_> {
    #[allow(clippy::let_unit_value)]
    #[allow(unused)]
    pub fn step(&mut self, direction: Direction) {
        //
    }
}
