use std::collections::HashSet;

use map::Map;
use walker::{Direction, Path, PathStatus, Position, Walker};

mod map;
pub mod walker;

pub const DIRECTIONS: [Direction; 4] = Direction::directions();

fn main() {
    let input = std::fs::read_to_string("./files/10.txt").unwrap();

    println!("Day 10");

    println!("Part 1: {}", part1(&input));
}

pub fn part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    let heads: Vec<Position> = map
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, digit)| (i, j, digit)))
        .filter(|(_, _, digit)| **digit == 0)
        .map(|(i, j, _)| Position { i, j })
        .collect();

    heads
        .into_iter()
        .map(|position| {
            let walker = Walker {
                position,
                map: &map,
                path: Path {
                    start: position,
                    directions: vec![],
                },
                status: PathStatus::Target(1),
            };

            let mut successful_walkers: Vec<Walker> = Vec::new();
            explore(walker, 9, &mut successful_walkers);
            HashSet::<Position>::from_iter(successful_walkers.iter().map(|walker| walker.position))
                .len()
        })
        .sum()
}

fn explore<'map>(walker: Walker<'map>, depth: usize, successful_walkers: &mut Vec<Walker<'map>>) {
    if depth == 0 {
        successful_walkers.push(walker.clone());
        return;
    }

    for direction in Direction::directions() {
        let mut walker_clone = walker.clone();
        if walker_clone.step(direction).is_ok() {
            explore(walker_clone, depth - 1, successful_walkers);
        }
    }
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1() {
        assert_eq!(36, super::part1(EXAMPLE));
    }
}
