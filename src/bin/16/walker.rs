use std::{
    collections::{HashSet, VecDeque},
    fmt::Write,
};

use crate::{
    direction::{self, Direction, Rotation},
    map::{Element, Map},
    position::Position,
};

#[derive(Debug, Clone)]
pub struct Solution<'map> {
    pub position: Position,
    pub map: &'map Map,
    pub start: Position,
    pub moves: Vec<Move>,
    pub direction: Direction,
}

impl Solution<'_> {
    pub fn score(&self) -> u32 {
        self.moves
            .iter()
            .map(|m| match m {
                Move::Step(_) => 1,
                Move::Rotate90Degree(_) => 1000,
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct PrintMap(pub Vec<Vec<PrintMapElement>>);

impl std::fmt::Display for PrintMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last_row_i = self.0.iter().len() - 1;
        for (i, row) in self.0.iter().enumerate() {
            for el in row.iter() {
                write!(f, "{el}")?;
            }

            if i != last_row_i {
                f.write_char('\n')?
            };
        }

        Ok(())
    }
}

impl From<Map> for PrintMap {
    fn from(value: Map) -> Self {
        PrintMap(
            value
                .0
                .into_iter()
                .map(|row| row.into_iter().map(PrintMapElement::from).collect())
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrintMapElement {
    Empty,
    Wall,
    Start,
    End,
    Direction(Direction),
}

impl PrintMapElement {
    fn as_char(&self) -> char {
        match self {
            PrintMapElement::Empty => '.',
            PrintMapElement::Wall => '#',
            PrintMapElement::Start => 'S',
            PrintMapElement::End => 'E',
            PrintMapElement::Direction(direction) => direction.as_char(),
        }
    }
}

impl std::fmt::Display for PrintMapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl From<Element> for PrintMapElement {
    fn from(value: Element) -> Self {
        match value {
            Element::Empty => PrintMapElement::Empty,
            Element::Wall => PrintMapElement::Wall,
            Element::Start => PrintMapElement::Start,
            Element::End => PrintMapElement::End,
        }
    }
}

impl std::fmt::Display for Solution<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = PrintMap::from(self.map.clone());

        let mut pos = self.start;
        let mut dir = Direction::Right;
        for movee in &self.moves {
            match movee {
                Move::Step(direction) => {
                    pos = direction::next_position(pos, *direction).unwrap();
                    map.0[pos.y][pos.x] = PrintMapElement::Direction(dir);
                }
                Move::Rotate90Degree(rotation) => {
                    dir = direction::rotate_90deg(dir, *rotation);
                }
            }
        }

        write!(f, "{map}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Visited {
    pub position: Position,
    pub direction: Direction,
    pub a_move: Move,
}

impl Solution<'_> {
    pub fn explore_solutions(&self) -> Vec<Solution> {
        let mut complete: Vec<Solution> = vec![];
        let mut queue: VecDeque<Solution> = VecDeque::from_iter(vec![self.clone()]);
        let mut visited: HashSet<Visited> = HashSet::new();

        while !queue.is_empty() && complete.len() < 5 {
            let mut solution = queue.pop_front().unwrap();

            let Some(next_position) =
                direction::next_position(solution.position, solution.direction)
            else {
                continue;
            };
            let Some(el) = solution.map.get(next_position) else {
                continue;
            };

            match el {
                Element::Empty => {
                    let mut solution = solution.clone();
                    solution.position = next_position;
                    let a_move = Move::Step(solution.direction);
                    solution.moves.push(a_move);

                    let visited_entry = Visited {
                        position: solution.position,
                        direction: solution.direction,
                        a_move,
                    };

                    if !visited.contains(&visited_entry) {
                        queue.push_back(solution);
                    }

                    visited.insert(visited_entry);
                }
                Element::Wall => {}
                Element::Start => {}
                Element::End => {
                    solution.position = next_position;
                    let a_move = Move::Step(solution.direction);
                    solution.moves.push(a_move);
                    complete.push(solution);
                    continue;
                }
            }

            direction::ROTATIONS.into_iter().for_each(|rotation| {
                let next_direction = direction::rotate_90deg(solution.direction, rotation);
                let mut solution = solution.clone();
                solution.direction = next_direction;
                let a_move = Move::Rotate90Degree(rotation);
                solution.moves.push(a_move);

                let visited_entry = Visited {
                    position: solution.position,
                    direction: solution.direction,
                    a_move,
                };

                if !visited.contains(&visited_entry) {
                    queue.push_back(solution);
                }

                visited.insert(visited_entry);
            });
        }

        complete
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Step(Direction),
    Rotate90Degree(Rotation),
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::{
        direction::Direction,
        map::{Map, MAP_EXAMPLE},
    };

    #[test]
    fn explore_solutions() {
        let map: Map = MAP_EXAMPLE.parse().unwrap();
        let start = map.find_start_position().unwrap();
        let solution = Solution {
            position: start,
            map: &map,
            start,
            moves: vec![],
            direction: Direction::Right,
        };

        let complete_solutions = solution.explore_solutions();
        let min = complete_solutions.iter().map(|s| s.score()).min().unwrap();
        assert_eq!(7036, min);
    }
}
