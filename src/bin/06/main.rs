use guard::{Guard, Position};
use map::{Element, Map};

mod guard;
mod map;
fn main() {
    let input = std::fs::read_to_string("./files/06.txt").unwrap();

    println!("Day 6");
    println!("Part 1: {}", part1(&input))
}

pub fn part1(map_input: &str) -> usize {
    let mut map: Map = map_input.parse().unwrap();

    let (i, j, direction) = map
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, element)| (i, j, element))
        })
        .find_map(|(i, j, element)| {
            if let Element::Guard(guard_element) = element {
                Some((i, j, guard_element.direction))
            } else {
                None
            }
        })
        .expect("Map has no guard");

    let mut guard = Guard {
        position: Position::Map(i, j),
        map: &mut map,
        direction,
    };

    while guard.step().is_ok() {}

    map.0
        .iter()
        .flat_map(|row| row.iter())
        .filter(|element| **element == Element::Visited)
        .collect::<Vec<_>>()
        .len()
}

#[cfg(test)]
mod test {

    const EXAMPLE_DATA: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        assert_eq!(41, super::part1(EXAMPLE_DATA));
    }
}
