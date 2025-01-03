use std::collections::{HashSet, VecDeque};

use super::map::{BoxEl, Element, Map};
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
            Element::Box(box_part) => {
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

                                break;
                            }
                            Element::Wall => break,
                            Element::Box(_) => positions_for_swap.push(position.x),
                            Element::Robot => panic!("Never happens"),
                        }
                    }
                } else {
                    let position = next_position;

                    let box_part_position = BoxPartPosition {
                        part: box_part,
                        position,
                        level: 0,
                    };

                    let another_box_part_position = match box_part {
                        super::map::BoxEl::Opening => BoxPartPosition {
                            part: BoxEl::Closing,
                            position: Position {
                                x: position.x + 1,
                                y: position.y,
                            },
                            level: 0,
                        },
                        super::map::BoxEl::Closing => BoxPartPosition {
                            part: BoxEl::Opening,
                            position: Position {
                                x: position.x - 1,
                                y: position.y,
                            },
                            level: 0,
                        },
                    };

                    let mut visited: HashSet<BoxPartPosition> = HashSet::new();
                    let mut queue: VecDeque<BoxPartPosition> =
                        VecDeque::from(vec![box_part_position, another_box_part_position]);

                    while !queue.is_empty() {
                        let box_el = queue.pop_front().unwrap();
                        let next_level = box_el.level + 1;
                        if visited.contains(&box_el) {
                            continue;
                        }
                        visited.insert(box_el);

                        let position = direction::next_position(box_el.position, direction).ok_or(
                            StepError {
                                map: self.map.to_owned(),
                                steps_made: self.steps_made,
                                kind: StepErrorKind::CouldNotComputeNextPosition {
                                    robot_position_before: next_position,
                                    direction,
                                },
                            },
                        )?;

                        let Some(next_el) = self.map.get(position) else {
                            continue;
                        };

                        match next_el {
                            Element::Empty => {}
                            Element::Wall => {}
                            Element::Box(next_box_part) => {
                                let next_y = match direction {
                                    Direction::Up => box_el.position.y.checked_sub(1),
                                    Direction::Down => Some(box_el.position.y + 1),
                                    _ => panic!("Left or Right never occur here"),
                                };
                                let Some(next_y) = next_y else {
                                    continue;
                                };
                                let x = box_el.position.x;

                                if box_el.part == next_box_part {
                                    queue.push_back(BoxPartPosition {
                                        part: next_box_part,
                                        position: Position {
                                            x: box_el.position.x,
                                            y: next_y,
                                        },
                                        level: next_level,
                                    });
                                } else {
                                    match box_el.part {
                                        BoxEl::Opening => {
                                            let opening = BoxPartPosition {
                                                part: BoxEl::Opening,
                                                position: Position {
                                                    x: x - 1,
                                                    y: next_y,
                                                },
                                                level: next_level,
                                            };
                                            let closing = BoxPartPosition {
                                                part: BoxEl::Closing,
                                                position: Position { x, y: next_y },
                                                level: next_level,
                                            };
                                            queue.push_back(opening);
                                            queue.push_back(closing);
                                        }
                                        BoxEl::Closing => {
                                            let opening = BoxPartPosition {
                                                part: BoxEl::Opening,
                                                position: Position { x, y: next_y },
                                                level: next_level,
                                            };
                                            let closing = BoxPartPosition {
                                                part: BoxEl::Closing,
                                                position: Position {
                                                    x: x + 1,
                                                    y: next_y,
                                                },
                                                level: next_level,
                                            };
                                            queue.push_back(opening);
                                            queue.push_back(closing);
                                        }
                                    }
                                }
                            }
                            Element::Robot => panic!("Never happens"),
                        }
                    }

                    let mut visited: Vec<BoxPartPosition> = Vec::from_iter(visited);
                    visited.sort_by(|a, b| b.level.cmp(&a.level));
                    let max_level = visited[0].level;

                    let can_move = visited.iter().filter(|el| el.level == max_level).all(
                        |BoxPartPosition { position, .. }| {
                            direction::next_position(*position, direction)
                                .and_then(|next_position| {
                                    self.map.get(next_position).and_then(|el| match el {
                                        Element::Empty => Some(true),
                                        Element::Wall => None,
                                        Element::Box(_) => Some(true),
                                        Element::Robot => panic!("Never ever happens"),
                                    })
                                })
                                .is_some()
                        },
                    );

                    if can_move {
                        visited
                            .iter()
                            .for_each(|BoxPartPosition { part, position, .. }| {
                                let Position { x, y } =
                                    direction::next_position(*position, direction).unwrap();
                                self.map.0[y][x] = Element::Box(*part);
                                self.map.0[position.y][position.x] = Element::Empty;
                            });
                        let next_robot_position =
                            direction::next_position(self.position, direction).unwrap();
                        self.position = next_robot_position;
                        self.map.0[robot_position_before.y][robot_position_before.x] =
                            Element::Empty;
                        self.map.0[self.position.y][self.position.x] = Element::Robot;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoxPartPosition {
    pub part: BoxEl,
    pub position: Position,
    pub level: usize,
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

    #[test]
    fn push_boxes_up() {
        let mut map: Map = "...##
.....
[][].
.[]..
..@..
"
        .parse()
        .unwrap();

        let mut robot = Robot {
            position: map.find_robot_position().unwrap(),
            map: &mut map,
            steps_made: 0,
        };

        robot.step(Direction::Up).unwrap();
        assert_eq!(
            "...##
[][].
.[]..
..@..
.....",
            robot.map.to_string().as_str()
        );

        robot.step(Direction::Up).unwrap();
        assert_eq!(
            "...##
[][].
.[]..
..@..
.....",
            robot.map.to_string().as_str()
        );
    }

    #[test]
    fn push_boxes_down() {
        let mut map: Map = "..@..
.[]..
[][].
.....
...##
"
        .parse()
        .unwrap();

        let mut robot = Robot {
            position: map.find_robot_position().unwrap(),
            map: &mut map,
            steps_made: 0,
        };

        robot.step(Direction::Down).unwrap();
        assert_eq!(
            ".....
..@..
.[]..
[][].
...##",
            robot.map.to_string().as_str()
        );

        robot.step(Direction::Down).unwrap();
        assert_eq!(
            ".....
..@..
.[]..
[][].
...##",
            robot.map.to_string().as_str()
        );
    }
}
