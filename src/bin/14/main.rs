use grid::Grid;
use size::Size;

pub mod grid;
pub mod position;
pub mod robot;
pub mod size;

fn main() {
    let input = std::fs::read_to_string("./files/14.txt").unwrap();

    println!("Day 14");

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::from_robots_input(
        input,
        Size {
            width: 101,
            height: 103,
        },
    );

    grid.wait_secs(100);
    grid.product_of_quadrants_robots_counts().unwrap()
}
