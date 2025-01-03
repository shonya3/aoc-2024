use crate::{
    direction::{self, Direction},
    map::{Element, Map},
    position::Position,
};

#[derive(Debug)]
pub struct Robot<'map> {
    pub position: Position,
    pub map: &'map mut Map,
    pub steps_made: u32,
}

#[derive(Debug)]
pub enum StepErrorKind {
    CouldNotComputeNextPosition {
        position_before: Position,
        direction: Direction,
    },
    UnexpectedAnotherRobot {
        position: Position,
    },
}

pub struct StepError {
    pub map: Map,
    pub steps_made: u32,
    pub kind: StepErrorKind,
}

impl std::fmt::Debug for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StepError")
            .field("map", &format_args!("{}", self.map))
            .field("steps_made", &self.steps_made)
            .field("kind", &self.kind)
            .finish()
    }
}

impl Robot<'_> {
    pub fn step(&mut self, direction: Direction) -> Result<(), StepError> {
        let robot_position_before = self.position;
        let next_position = direction::next_position(self.position, direction).unwrap();
        let el = self.map.get(next_position).unwrap();
        match el {
            crate::map::Element::Empty => {
                // println!(
                //     "Down. Position before: {position_before:?}. Current position: {next_position:?}"
                // );
                self.map.0[next_position.y][next_position.x] = Element::Robot;
                self.position = next_position;
                self.map.0[robot_position_before.y][robot_position_before.x] = Element::Empty;
            }
            crate::map::Element::Wall => {}
            crate::map::Element::Box => {
                let mut position = next_position;
                loop {
                    position = direction::next_position(position, direction).ok_or(StepError {
                        map: self.map.to_owned(),
                        steps_made: self.steps_made,
                        kind: StepErrorKind::CouldNotComputeNextPosition {
                            position_before: position,
                            direction,
                        },
                    })?;

                    let el = self.map.get(position).unwrap();
                    match el {
                        Element::Empty => {
                            self.map.0[position.y][position.x] = Element::Box;
                            self.map.0[next_position.y][next_position.x] = Element::Robot;
                            self.position = next_position;
                            self.map.0[robot_position_before.y][robot_position_before.x] =
                                Element::Empty;
                            break;
                        }
                        Element::Wall => break,
                        Element::Box => {}
                        Element::Robot => panic!("Never happens"),
                    }
                }
            }
            crate::map::Element::Robot => {
                return Err(StepError {
                    map: self.map.to_owned(),
                    steps_made: self.steps_made,
                    kind: StepErrorKind::UnexpectedAnotherRobot {
                        position: next_position,
                    },
                });
            }
        }

        self.steps_made += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{direction::Direction, map::Map};

    use super::Robot;

    #[test]
    fn push_boxes() {
        let mut map: Map = "#...OO@".parse().unwrap();
        let mut robot = Robot {
            position: map.find_robot_position().unwrap(),
            map: &mut map,
            steps_made: 0,
        };

        robot.step(Direction::Left).unwrap();
        assert_eq!("#..OO@.", robot.map.to_string().as_str());
        robot.step(Direction::Left).unwrap();
        assert_eq!("#.OO@..", robot.map.to_string().as_str());
        robot.step(Direction::Left).unwrap();
        assert_eq!("#OO@...", robot.map.to_string().as_str());
        robot.step(Direction::Left).unwrap();
        assert_eq!("#OO@...", robot.map.to_string().as_str());
    }
}
