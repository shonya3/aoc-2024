use map::Map;
use std::collections::HashSet;
use walker::{Direction, Path, PathStatus, Position, Walker};

mod map;
mod walker;

fn main() {
    let input = std::fs::read_to_string("./files/10.txt").unwrap();

    println!("Day 10");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

            let successful_walkers = explore(walker);
            HashSet::<Position>::from_iter(successful_walkers.iter().map(|walker| walker.position))
                .len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
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

            explore(walker).len()
        })
        .sum()
}

fn explore(walker: Walker) -> Vec<Walker> {
    let mut successful_walkers = Vec::new();
    if walker.status == PathStatus::Done {
        successful_walkers.push(walker);
        return successful_walkers;
    }

    for direction in Direction::directions() {
        let mut walker = walker.clone();
        if walker.step(direction).is_ok() {
            let walkers = explore(walker);
            successful_walkers.extend(walkers);
        }
    }

    successful_walkers
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

    #[test]
    fn part2() {
        assert_eq!(81, super::part2(EXAMPLE));
    }
}
