use grid::{Element, Grid};
use size::Size;

pub mod grid;
pub mod position;
pub mod robot;
pub mod size;

fn main() {
    let input = std::fs::read_to_string("./files/14.txt").unwrap();

    println!("Day 14");

    println!("Part 1: {}", part1(&input));
    let (grid, seconds_passed) = part2(&input);
    println!("Part 2: {grid}\nSeconds passed: {seconds_passed}");
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

fn part2(input: &str) -> (Grid, u32) {
    let mut grid = Grid::from_robots_input(
        input,
        Size {
            width: 101,
            height: 103,
        },
    );

    let mut seconds_passed = 0;

    loop {
        for (x, y) in grid.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, el)| match el {
                Element::Empty => None,
                Element::Robots(_) => Some((x, y)),
            })
        }) {
            let mut n = 0;
            let mut y = y;

            loop {
                y += 1;
                let Some(el) = grid.grid.get(y).and_then(|y| y.get(x)) else {
                    break;
                };

                match el {
                    Element::Empty => break,
                    Element::Robots(_) => n += 1,
                }

                if n == 10 {
                    return (grid, seconds_passed);
                }
            }
        }

        grid.wait_one_second();
        seconds_passed += 1;
    }
}
