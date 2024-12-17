use std::collections::HashMap;

use map::{AntennaId, Element, Map};

mod map;

fn main() {
    let input = std::fs::read_to_string("./files/08.txt").unwrap();

    println!("Day 8");

    println!("Part 1: {}", part1(&input));
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

fn part1(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();
    let mut antennas: HashMap<AntennaId, Vec<Position>> = HashMap::new();

    map.0.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, element)| {
            if let Element::Antenna(id) = element {
                antennas.entry(*id).or_default().push(Position { i, j });
            }
        })
    });

    let mut n = 0;

    antennas.into_iter().for_each(|(_, positions)| {
        let pairs = positions
            .iter()
            .enumerate()
            .flat_map(|(i, a)| positions.iter().skip(i + 1).map(move |b| (*a, *b)))
            .collect::<Vec<(Position, Position)>>();

        pairs.into_iter().for_each(|(a, b)| {
            let diff = (b.i.abs_diff(a.i), b.j.abs_diff(a.j));

            let antinode_a = (
                match a.i < b.i {
                    true => a.i as isize - diff.0 as isize,
                    false => a.i as isize + diff.0 as isize,
                },
                match a.j < b.j {
                    true => a.j as isize - diff.1 as isize,
                    false => a.j as isize + diff.1 as isize,
                },
            );
            let antinode_b = (
                match b.i > a.i {
                    true => b.i as isize + diff.0 as isize,
                    false => b.i as isize - diff.0 as isize,
                },
                match b.j > a.j {
                    true => b.j as isize + diff.1 as isize,
                    false => b.j as isize - diff.1 as isize,
                },
            );

            if let Some(el) = map.get(antinode_a.0 as usize, antinode_a.1 as usize) {
                if el != Element::Antinode {
                    n += 1;
                }

                if el == Element::Empty {
                    map.0[antinode_a.0 as usize][antinode_a.1 as usize] = Element::Antinode;
                }
            }

            if let Some(el) = map.get(antinode_b.0 as usize, antinode_b.1 as usize) {
                if el != Element::Antinode {
                    n += 1;
                }

                if el == Element::Empty {
                    map.0[antinode_b.0 as usize][antinode_b.1 as usize] = Element::Antinode;
                }
            }
        });
    });

    n
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        assert_eq!(14, super::part1(input));
    }
}
