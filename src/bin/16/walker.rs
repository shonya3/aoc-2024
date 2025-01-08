#![allow(unused)]

use std::collections::{HashMap, HashSet, VecDeque};

use solution_map::SolutionMap;
use tile_map::TileMap;

use crate::{
    direction::{self, Direction, Rotation},
    map::{Element, Map},
    position::{self, Position},
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

    pub fn score_at_position_and_direction(&self, position: Position, direction: Direction) -> u32 {
        let mut score = 0;

        let mut pos = self.start;
        let mut dir = Direction::Right;
        for movee in &self.moves {
            if pos == position && dir == direction {
                break;
            }

            match movee {
                Move::Step(direction) => {
                    pos = direction::next_position(pos, *direction).unwrap();
                    score += 1;
                }
                Move::Rotate90Degree(rotation) => {
                    dir = direction::rotate_90deg(dir, *rotation);
                    score += 1000;
                }
            }
        }

        score
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scored {
    score: u32,
    direction: Direction,
    position: Position,
}

impl Solution<'_> {
    pub fn explore_part2(&self) -> usize {
        let mut complete: Vec<Solution> = vec![];
        let mut queue: VecDeque<Solution> = VecDeque::from_iter(vec![self.clone()]);
        let mut visited: HashMap<Visited, u32> = HashMap::new();
        let mut scored: HashMap<Scored, Vec<Solution>> = HashMap::new();

        while let Some(solution) = queue.pop_front() {
            if let Some(Element::End) = solution.map.get(solution.position) {
                complete.push(solution.clone());
                continue;
            }

            let pos = Position { x: 3, y: 10 };
            if solution.position == pos && solution.direction == Direction::Up {
                println!("HEHREREE {}", solution.score());
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
                            .map_or(true, |&prev_score| current_score <= prev_score)
                        {
                            visited.insert(visited_entry, current_score);
                            scored
                                .entry(Scored {
                                    direction: new_solution.direction,
                                    score: current_score,
                                    position: new_solution.position,
                                })
                                .or_default()
                                .push(new_solution.clone());
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
                    .map_or(true, |&prev_score| current_score <= prev_score)
                {
                    visited.insert(visited_entry, current_score);
                    scored
                        .entry(Scored {
                            direction: new_solution.direction,
                            score: current_score,
                            position: new_solution.position,
                        })
                        .or_default()
                        .push(new_solution.clone());
                    queue.push_back(new_solution);
                }
            }
        }

        complete.sort_by_key(|a| a.score());
        let first_min_solution = complete.first().unwrap();
        let first_min_solution_score = complete.first().unwrap().score();

        let positions = complete
            .clone()
            .into_iter()
            .take_while(|solution| solution.score() == first_min_solution_score)
            .flat_map(|solution| SolutionMap::from(solution).steps_positions())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        // let tile_map = TileMap::new(first_min_solution.map.clone(), positions.clone());
        positions.len()
    }

    pub fn explore_solutions(&self) -> Vec<Solution> {
        let mut complete: Vec<Solution> = vec![];
        let mut queue: VecDeque<Solution> = VecDeque::from_iter(vec![self.clone()]);
        let mut visited: HashMap<Visited, u32> = HashMap::new();
        let mut scored: HashMap<Position, Vec<Solution>> = HashMap::new();

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
                            scored
                                .entry(new_solution.position)
                                .or_default()
                                .push(new_solution.clone());
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
                    scored
                        .entry(new_solution.position)
                        .or_default()
                        .push(new_solution.clone());
                    queue.push_back(new_solution);
                }
            }
        }

        // 3,9
        let solutions = scored.get(&Position { x: 3, y: 9 }).unwrap().clone();
        println!("{:#?}", solutions.len());

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

mod tile_map {
    use crate::{
        map::{Element, Map},
        position::Position,
    };
    use std::fmt::Write;

    #[derive(Debug, Clone)]
    pub struct TileMap(pub Vec<Vec<TileMapElement>>);

    impl TileMap {
        pub fn new(mut map: Map, tiles_positions: Vec<Position>) -> TileMap {
            let mut map = TileMap::from(map);
            tiles_positions
                .into_iter()
                .for_each(|Position { x, y }| map.0[y][x] = TileMapElement::Tile);
            map
        }
    }

    impl std::fmt::Display for TileMap {
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

    impl From<Map> for TileMap {
        fn from(value: Map) -> Self {
            TileMap(
                value
                    .0
                    .into_iter()
                    .map(|row| row.into_iter().map(TileMapElement::from).collect())
                    .collect(),
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TileMapElement {
        Empty,
        Wall,
        Start,
        End,
        Tile,
    }

    impl TileMapElement {
        fn as_char(&self) -> char {
            match self {
                TileMapElement::Empty => '.',
                TileMapElement::Wall => '#',
                TileMapElement::Start => 'S',
                TileMapElement::End => 'E',
                TileMapElement::Tile => 'O',
            }
        }
    }

    impl std::fmt::Display for TileMapElement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char(self.as_char())
        }
    }

    impl From<Element> for TileMapElement {
        fn from(value: Element) -> Self {
            match value {
                Element::Empty => TileMapElement::Empty,
                Element::Wall => TileMapElement::Wall,
                Element::Start => TileMapElement::Start,
                Element::End => TileMapElement::End,
            }
        }
    }
}

mod solution_map {
    use super::Solution;
    use crate::{
        direction::{self, Direction},
        map::{Element, Map},
        position::Position,
        walker::Move,
    };
    use std::fmt::Write;

    #[derive(Debug, Clone)]
    pub struct SolutionMap(pub Vec<Vec<SolutionMapElement>>);

    impl SolutionMap {
        pub fn steps_positions_directions(&self) -> Vec<(Position, Direction)> {
            self.0
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, el)| {
                        if let SolutionMapElement::Direction(dir) = el {
                            return Some((Position { x, y }, *dir));
                        }

                        None
                    })
                })
                .collect()
        }

        pub fn steps_positions(&self) -> Vec<Position> {
            self.0
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, el)| {
                        if let SolutionMapElement::Direction(_) = el {
                            return Some(Position { x, y });
                        }

                        None
                    })
                })
                .collect()
        }
    }

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
                        map.0[pos.y][pos.x] = SolutionMapElement::Direction(dir);
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
        map::{Map, MAP_EXAMPLE, MAP_EXAMPLE2},
        position::Position,
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

        let mut complete_solutions = solution.explore_solutions();

        let min = complete_solutions.iter().map(|s| s.score()).min().unwrap();
        assert_eq!(7036, min);

        complete_solutions.sort_by_key(|a| a.score());
        assert_eq!(
            3006,
            complete_solutions[0]
                .score_at_position_and_direction(Position { x: 3, y: 9 }, Direction::Up)
        )
    }

    #[test]
    fn explore_part2() {
        let map: Map = MAP_EXAMPLE.parse().unwrap();
        let start = map.find_start_position().unwrap();
        let solution = Solution {
            position: start,
            map: &map,
            start,
            moves: vec![],
            direction: Direction::Right,
        };
        assert_eq!(45, solution.explore_part2());

        let map: Map = MAP_EXAMPLE2.parse().unwrap();
        let start = map.find_start_position().unwrap();
        let solution = Solution {
            position: start,
            map: &map,
            start,
            moves: vec![],
            direction: Direction::Right,
        };
        assert_eq!(64, solution.explore_part2());
    }
}
