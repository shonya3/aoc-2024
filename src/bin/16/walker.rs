use std::collections::{HashMap, VecDeque};

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

impl Solution<'_> {
    pub fn explore_solutions(&self) -> Vec<Solution> {
        let mut complete: Vec<Solution> = vec![];
        let mut queue: VecDeque<Solution> = VecDeque::from_iter(vec![self.clone()]);
        let mut visited: HashMap<Visited, u32> = HashMap::new();

        while let Some(solution) = queue.pop_front() {
            if let Some(Element::End) = solution.map.get(solution.position) {
                complete.push(solution.clone());
                continue;
            }

            if let Some(next_position) =
                direction::next_position(solution.position, solution.direction)
            {
                if let Some(el) = solution.map.get(next_position) {
                    if el != Element::Wall {
                        let mut new_solution = solution.clone();
                        new_solution.position = next_position;
                        new_solution.moves.push(Move::Step(new_solution.direction));
                        let current_score = new_solution.score();
                        let visited_entry = Visited {
                            position: new_solution.position,
                            direction: new_solution.direction,
                        };

                        if visited
                            .get(&visited_entry)
                            .map_or(true, |&prev_score| current_score < prev_score)
                        {
                            visited.insert(visited_entry, current_score);
                            queue.push_back(new_solution);
                        }
                    }
                }
            }

            for &rotation in direction::ROTATIONS.iter() {
                let new_direction = direction::rotate_90deg(solution.direction, rotation);
                let mut new_solution = solution.clone();
                new_solution.direction = new_direction;
                new_solution.moves.push(Move::Rotate90Degree(rotation));
                let current_score = new_solution.score();
                let visited_entry = Visited {
                    position: new_solution.position,
                    direction: new_solution.direction,
                };

                if visited
                    .get(&visited_entry)
                    .map_or(true, |&prev_score| current_score < prev_score)
                {
                    visited.insert(visited_entry, current_score);
                    queue.push_back(new_solution);
                }
            }
        }

        complete
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Visited {
    pub position: Position,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Step(Direction),
    Rotate90Degree(Rotation),
}

impl std::fmt::Display for Solution<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        solution_map::SolutionMap::from(self.clone()).fmt(f)
    }
}

mod solution_map {
    use super::Solution;
    use crate::{
        direction::{self, Direction},
        map::{Element, Map},
        walker::Move,
    };
    use std::fmt::Write;

    #[derive(Debug, Clone)]
    pub struct SolutionMap(pub Vec<Vec<SolutionMapElement>>);

    impl From<Solution<'_>> for SolutionMap {
        fn from(solution: Solution) -> Self {
            let mut map = SolutionMap::from(solution.map.clone());

            let mut pos = solution.start;
            let mut dir = Direction::Right;
            for movee in &solution.moves {
                match movee {
                    Move::Step(direction) => {
                        pos = direction::next_position(pos, *direction).unwrap();
                        map.0[pos.y][pos.x] = SolutionMapElement::Direction(dir);
                    }
                    Move::Rotate90Degree(rotation) => {
                        dir = direction::rotate_90deg(dir, *rotation);
                    }
                }
            }

            map
        }
    }

    impl std::fmt::Display for SolutionMap {
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

    impl From<Map> for SolutionMap {
        fn from(value: Map) -> Self {
            SolutionMap(
                value
                    .0
                    .into_iter()
                    .map(|row| row.into_iter().map(SolutionMapElement::from).collect())
                    .collect(),
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum SolutionMapElement {
        Empty,
        Wall,
        Start,
        End,
        Direction(Direction),
    }

    impl SolutionMapElement {
        fn as_char(&self) -> char {
            match self {
                SolutionMapElement::Empty => '.',
                SolutionMapElement::Wall => '#',
                SolutionMapElement::Start => 'S',
                SolutionMapElement::End => 'E',
                SolutionMapElement::Direction(direction) => direction.as_char(),
            }
        }
    }

    impl std::fmt::Display for SolutionMapElement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char(self.as_char())
        }
    }

    impl From<Element> for SolutionMapElement {
        fn from(value: Element) -> Self {
            match value {
                Element::Empty => SolutionMapElement::Empty,
                Element::Wall => SolutionMapElement::Wall,
                Element::Start => SolutionMapElement::Start,
                Element::End => SolutionMapElement::End,
            }
        }
    }
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
