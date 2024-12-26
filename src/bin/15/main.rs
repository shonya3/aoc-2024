use input::Input;
use robot::Robot;

pub mod direction;
pub mod input;
pub mod map;
pub mod position;
pub mod robot;

fn main() {
    let input = std::fs::read_to_string("./files/15.txt").unwrap();

    println!("Day 15");

    part1(&input);
}

fn part1(input: &str) {
    let Input {
        mut map,
        directions,
    } = input.parse().unwrap();

    let mut robot = Robot {
        position: map.find_robot_position().unwrap(),
        map: &mut map,
        steps_made: 0,
    };

    println!("Initial map: {}", robot.map);

    for direction in directions {
        println!("\n\nStep {} Move {direction:#?}", robot.steps_made + 1);
        if let Err(err) = robot.step(direction) {
            println!("{err:#?}");
            break;
        }

        println!("{}", robot.map);
    }
}

#[cfg(test)]
mod tests {
    use crate::{input::INPUT_EXAMPLE, part1};

    #[test]
    fn p1() {
        part1(INPUT_EXAMPLE);
    }
}
