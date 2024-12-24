use crate::{position::Position, size::Size};

#[derive(Debug, Clone)]
pub struct Robot {
    pub position: Position,
    pub velocity: Velocity,
    pub grid_size: Size,
}

pub fn parse_input(input: &str) -> (Position, Velocity) {
    let (p, v) = input.split_once(" ").unwrap();
    let position = p
        .replace("p=", "")
        .trim()
        .split_once(",")
        .map(|(left, right)| Position {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        })
        .unwrap();

    let velocity = v
        .replace("v=", "")
        .trim()
        .split_once(",")
        .map(|(left, right)| Velocity {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        })
        .unwrap();

    (position, velocity)
}

impl Robot {
    pub fn from_input(input: &str, grid_size: Size) -> Vec<Robot> {
        input
            .lines()
            .map(|line| Robot::from_single_line_input(line, grid_size.clone()))
            .collect()
    }

    fn from_single_line_input(input: &str, grid_size: Size) -> Robot {
        let (position, velocity) = parse_input(input);
        Robot {
            position,
            velocity,
            grid_size,
        }
    }

    pub fn move_one_second(&mut self) {
        self.position = next_position(&self.position, &self.velocity, &self.grid_size);
    }
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

pub fn next_position(position: &Position, velocity: &Velocity, grid_size: &Size) -> Position {
    let max_map_x = grid_size.width - 1;
    let max_map_y = grid_size.height - 1;

    let count_x = || {
        let mut x = position.x;

        for _ in 0..velocity.x.abs() {
            let sum = match velocity.x > 0 {
                true => x as i32 + 1,
                false => x as i32 - 1,
            };

            x = match sum {
                -1 => max_map_x,
                sum if sum as usize == max_map_x + 1 => 0,
                sum if sum < -1 => panic!("Never happens"),
                other => other as usize,
            };
        }

        x
    };

    let count_y = || {
        let mut y = position.y;

        for _ in 0..velocity.y.abs() {
            let sum = match velocity.y > 0 {
                true => y as i32 + 1,
                false => y as i32 - 1,
            };

            y = match sum {
                -1 => max_map_y,
                sum if sum as usize == max_map_y + 1 => 0,
                sum if sum < -1 => panic!("Never happens"),
                other => other as usize,
            };
        }

        y
    };

    Position {
        x: count_x(),
        y: count_y(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{position::Position, robot::Velocity, size::Size};

    #[test]
    fn next_position() {
        let mut position = Position { x: 2, y: 4 };
        let velocity = Velocity { x: 2, y: -3 };
        let grid_size = Size {
            width: 11,
            height: 7,
        };

        position = super::next_position(&position, &velocity, &grid_size);
        assert_eq!(Position { x: 4, y: 1 }, position);

        position = super::next_position(&position, &velocity, &grid_size);
        assert_eq!(Position { x: 6, y: 5 }, position);

        position = super::next_position(&position, &velocity, &grid_size);
        assert_eq!(Position { x: 8, y: 2 }, position);

        position = super::next_position(&position, &velocity, &grid_size);
        assert_eq!(Position { x: 10, y: 6 }, position);

        position = super::next_position(&position, &velocity, &grid_size);
        assert_eq!(Position { x: 1, y: 3 }, position);
    }
}
