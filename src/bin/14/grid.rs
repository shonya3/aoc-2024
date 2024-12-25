use crate::{position::Position, robot::Robot, size::Size};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub size: Size,
    pub grid: Vec<Vec<Element>>,
    pub robots: Vec<Robot>,
}

impl Grid {
    pub fn from_robots_input(input: &str, size: Size) -> Grid {
        let grid: Vec<Vec<Element>> = (0..size.height)
            .map(|_| (0..size.width).map(|_| Element::Empty).collect::<Vec<_>>())
            .collect();

        let robots = Robot::from_input(input, size.clone());

        let mut grid = Grid { size, grid, robots };
        grid.update_grid_elements();

        grid
    }
}

impl Grid {
    pub fn product_of_quadrants_robots_counts(&self) -> Option<u32> {
        let quadrants = self.quadrants()?;

        Some(
            quadrants[0].count_robots()
                * quadrants[1].count_robots()
                * quadrants[2].count_robots()
                * quadrants[3].count_robots(),
        )
    }

    pub fn quadrants(&self) -> Option<[Quadrant; 4]> {
        if self.size.width % 2 == 0 || self.size.height % 2 == 0 {
            return None;
        }

        let x_mid = self.size.width / 2;
        let y_mid = self.size.height / 2;

        // top-left
        let q1 = Quadrant {
            grid: self,
            start: Position { x: 0, y: 0 },
            end: Position {
                x: x_mid - 1,
                y: y_mid - 1,
            },
        };

        // top-right
        let q2 = Quadrant {
            grid: self,
            start: Position { x: x_mid + 1, y: 0 },
            end: Position {
                x: self.size.width - 1,
                y: y_mid - 1,
            },
        };

        // bottom-left
        let q3 = Quadrant {
            grid: self,
            start: Position { x: 0, y: y_mid + 1 },
            end: Position {
                x: x_mid - 1,
                y: self.size.height - 1,
            },
        };

        // bottom-right
        let q4 = Quadrant {
            grid: self,
            start: Position {
                x: x_mid + 1,
                y: y_mid + 1,
            },
            end: Position {
                x: self.size.width - 1,
                y: self.size.height - 1,
            },
        };

        Some([q1, q2, q3, q4])
    }

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

            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Empty,
    Robots(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quadrant<'grid> {
    pub grid: &'grid Grid,
    pub start: Position,
    pub end: Position,
}

impl Quadrant<'_> {
    pub fn count_robots(&self) -> u32 {
        let min_x = self.start.x;
        let min_y = self.start.y;
        let max_x = self.end.x;
        let max_y = self.end.y;

        self.grid
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, element)| (x, y, element))
            })
            .filter(|(x, y, _)| {
                if *x < min_x || *x > max_x {
                    return false;
                }

                if *y < min_y || *y > max_y {
                    return false;
                }

                true
            })
            .map(|(_, _, element)| match element {
                Element::Empty => 0,
                Element::Robots(n) => *n,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{Grid, Quadrant};
    use crate::{position::Position, robot::ROBOTS_INPUT_EXAMPLE, size::Size};

    #[test]
    fn quadrants() {
        let grid = Grid::from_robots_input(
            ROBOTS_INPUT_EXAMPLE,
            Size {
                width: 11,
                height: 7,
            },
        );

        let q1 = Quadrant {
            grid: &grid,
            start: Position { x: 0, y: 0 },
            end: Position { x: 4, y: 2 },
        };

        let q2 = Quadrant {
            grid: &grid,
            start: { Position { x: 6, y: 0 } },
            end: Position { x: 10, y: 2 },
        };

        let q3 = Quadrant {
            grid: &grid,
            start: Position { x: 0, y: 4 },
            end: Position { x: 4, y: 6 },
        };

        let q4 = Quadrant {
            grid: &grid,
            start: Position { x: 6, y: 4 },
            end: Position { x: 10, y: 6 },
        };

        let quadrants = grid.quadrants().unwrap();
        assert!(quadrants.iter().any(|q| *q == q1));
        assert!(quadrants.iter().any(|q| *q == q2));
        assert!(quadrants.iter().any(|q| *q == q3));
        assert!(quadrants.iter().any(|q| *q == q4));
    }

    #[test]
    fn product_of_quadrants_robots_counts() {
        let mut grid = Grid::from_robots_input(
            ROBOTS_INPUT_EXAMPLE,
            Size {
                width: 11,
                height: 7,
            },
        );
        grid.wait_secs(100);
        assert_eq!(Some(12), grid.product_of_quadrants_robots_counts());
    }
}
