use direction::Direction;
use map::Map;
use walker::Solution;

pub mod direction;
pub mod map;
pub mod position;
pub mod walker;

fn main() {
    let input = std::fs::read_to_string("./files/16.txt").unwrap();

    println!("Day 16");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> u32 {
    let map: Map = input.parse().unwrap();
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

    min
}

pub fn part2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    let start = map.find_start_position().unwrap();
    let solution = Solution {
        position: start,
        map: &map,
        start,
        moves: vec![],
        direction: Direction::Right,
    };

    solution.explore_part2()
}
