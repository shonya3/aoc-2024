use double::{map::Map as DoubleMap, robot::Robot as DoubleMapRobot};
use input::Input;
use robot::Robot;

pub mod direction;
pub mod double;
pub mod input;
pub mod map;
pub mod position;
pub mod robot;

fn main() {
    let input = std::fs::read_to_string("./files/15.txt").unwrap();

    println!("Day 15");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let Input {
        mut map,
        directions,
    } = input.parse().unwrap();

    let mut robot = Robot {
        position: map.find_robot_position().unwrap(),
        map: &mut map,
        steps_made: 0,
    };

    for direction in directions {
        robot.step(direction).unwrap();
    }

    robot.map.boxes_gps()
}

fn part2(input: &str) -> usize {
    let Input { map, directions } = input.parse().unwrap();
    let mut map = DoubleMap::from(map);
    let mut robot = DoubleMapRobot {
        position: map.find_robot_position().unwrap(),
        map: &mut map,
        steps_made: 0,
    };

    for direction in directions {
        robot.step(direction).unwrap();
    }

    robot.map.boxes_gps()
}

#[cfg(test)]
mod tests {
    use crate::{input::INPUT_EXAMPLE, part1, part2};

    #[test]
    fn p1() {
        assert_eq!(10092, part1(INPUT_EXAMPLE));
    }

    #[test]
    fn p2() {
        assert_eq!(9021, part2(INPUT_EXAMPLE))
    }
}
