use super::map::{Element, Map};
use crate::{
    direction::{self, Direction},
    position::Position,
};

#[derive(Debug)]
pub struct Robot<'map> {
    pub position: Position,
    pub map: &'map mut Map,
    pub steps_made: u32,
}
