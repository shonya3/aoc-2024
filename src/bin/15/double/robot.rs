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
            Element::Empty => {
                self.map.0[next_position.y][next_position.x] = Element::Robot;
                self.position = next_position;
                self.map.0[robot_position_before.y][robot_position_before.x] = Element::Empty;
            }
            Element::Wall => {}
            Element::Box(_) => {
                if matches!(direction, Direction::Left) || matches!(direction, Direction::Right) {
                    let y = robot_position_before.y;
                    let x = robot_position_before.x;

                    let mut positions_for_swap: Vec<usize> = vec![x, next_position.x];
                    let mut position = next_position;
                    loop {
                        position =
                            direction::next_position(position, direction).ok_or(StepError {
                                map: self.map.to_owned(),
                                steps_made: self.steps_made,
                                kind: StepErrorKind::CouldNotComputeNextPosition {
                                    robot_position_before: position,
                                    direction,
                                },
                            })?;

                        let el = self.map.get(position).unwrap();
                        match el {
                            Element::Empty => {
                                let row = self.map.0.get_mut(y).unwrap();
                                positions_for_swap.push(position.x);
                                positions_for_swap.sort();

                                let initial_pair_of_indexes_for_swap = match direction {
                                    Direction::Left => {
                                        positions_for_swap.first().cloned().map(|i| (i, i + 1))
                                    }
                                    Direction::Right => {
                                        positions_for_swap.last().cloned().map(|i| (i - 1, i))
                                    }
                                    _ => panic!("never happens"),
                                };
                                let Some((mut m, mut n)) = initial_pair_of_indexes_for_swap else {
                                    break;
                                };

                                let mut swaps_remaining = positions_for_swap.len() - 1;
                                loop {
                                    if swaps_remaining == 0 {
                                        break;
                                    }

                                    row.swap(n, m);
                                    match direction {
                                        Direction::Left => {
                                            n += 1;
                                            m += 1;
                                        }
                                        Direction::Right => {
                                            n -= 1;
                                            m -= 1;
                                        }
                                        _ => panic!("never happens"),
                                    }
                                    swaps_remaining -= 1;
                                }

                                self.position = self.map.find_robot_position().unwrap();

                                println!("{positions_for_swap:?}");
                                break;
                            }
                            Element::Wall => break,
                            Element::Box(_) => positions_for_swap.push(position.x),
                            Element::Robot => panic!("Never happens"),
                        }
                    }
                }
            }
            Element::Robot => {
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

#[derive(Debug)]
pub enum StepErrorKind {
    CouldNotComputeNextPosition {
        robot_position_before: Position,
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

#[cfg(test)]
mod tests {
    use super::Robot;
    use crate::{direction::Direction, double::map::Map};

    #[test]
    fn push_boxes_left() {
        let mut map: Map = "#..[][][]@".parse().unwrap();
        let mut robot = Robot {
            position: map.find_robot_position().unwrap(),
            map: &mut map,
            steps_made: 0,
        };

        robot.step(Direction::Left).unwrap();
        assert_eq!("#.[][][]@.", robot.map.to_string().as_str());
        robot.step(Direction::Left).unwrap();
        assert_eq!("#[][][]@..", robot.map.to_string().as_str());
    }

    #[test]
    fn push_boxes_right() {
        let mut map: Map = "#@[][][]..".parse().unwrap();
        let mut robot = Robot {
            position: map.find_robot_position().unwrap(),
            map: &mut map,
            steps_made: 0,
        };

        robot.step(Direction::Right).unwrap();
        assert_eq!("#.@[][][].", robot.map.to_string().as_str());
        robot.step(Direction::Right).unwrap();
        assert_eq!("#..@[][][]", robot.map.to_string().as_str());
    }
}
