use std::collections::HashMap;

use crate::{position::Position, robot::Robot, size::Size};

#[derive(Debug)]
pub struct Grid {
    pub size: Size,
    pub grid: Vec<Vec<Element>>,
    pub robots: Vec<Robot>,
}

impl Grid {
    pub fn from_input(input: &str, size: Size) -> Grid {
        let grid: Vec<Vec<Element>> = (0..size.height)
            .map(|_| (0..size.width).map(|_| Element::Empty).collect::<Vec<_>>())
            .collect();

        // println!("grid height: {}", grid.len());
        // println!("grid width: {}", grid[0].len());

        let robots = Robot::from_input(input, size.clone());

        let mut grid = Grid { size, grid, robots };
        grid.update_grid_elements();

        grid
    }
}

impl Grid {
    pub fn wait_secs(&mut self, secs: u32) {
        for _ in 0..secs {
            self.wait_one_second();
        }
    }

    pub fn wait_one_second(&mut self) {
        self.robots.iter_mut().for_each(|robot| {
            robot.move_one_second();
        });

        self.update_grid_elements();
    }

    fn update_grid_elements(&mut self) {
        let mut robot_positions: HashMap<Position, u32> = HashMap::new();
        self.robots.iter().for_each(|robot| {
            *robot_positions.entry(robot.position).or_default() += 1;
        });

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let position = Position { x, y };
                self.grid[y][x] = match robot_positions.get(&position) {
                    Some(number_of_robots) => Element::Robots(*number_of_robots),
                    None => Element::Empty,
                };
            }
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for element in row.iter() {
                match element {
                    Element::Empty => f.write_str(".")?,
                    Element::Robots(n) => write!(f, "{n}")?,
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Element {
    Empty,
    Robots(u32),
}
