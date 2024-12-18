use map::{Element, Map};

pub mod map;

fn main() {
    let input = std::fs::read_to_string("./files/09.txt").unwrap();
    println!("Day 9");

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u128 {
    Map::from_input(input)
        .unwrap()
        .compress()
        .0
        .iter()
        .enumerate()
        .map(|(i, el)| match el {
            Element::File(file_id) => i as u128 * file_id.0,
            Element::Empty => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1() {
        assert_eq!(1928, super::part1(EXAMPLE))
    }
}
