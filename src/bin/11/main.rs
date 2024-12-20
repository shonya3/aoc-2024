use std::num::ParseIntError;

fn main() {
    let input = "4 4841539 66 5279 49207 134 609568 0";

    println!("Day 11");

    println!("Part 1: {}", part1(input));
}

pub fn part1(input: &str) -> usize {
    let mut stones = parse_input(input).unwrap();

    for _ in 0..25 {
        stones = split_stones(&stones)
    }

    stones.len()
}

fn parse_input(input: &str) -> Result<Vec<Stone>, ParseU64Error> {
    input
        .split(" ")
        .map(|s| {
            s.parse::<u64>()
                .map(Stone)
                .map_err(|err| ParseU64Error(s.to_owned(), err))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stone(pub u64);

pub fn split_stones(stones: &[Stone]) -> Vec<Stone> {
    stones
        .iter()
        .flat_map(|stone| split_stone(*stone))
        .collect()
}

pub fn split_stone(stone: Stone) -> Vec<Stone> {
    let mut stones: Vec<Stone> = Vec::new();

    match stone.0 {
        0 => stones.push(Stone(1)),
        value => match split_on_two(value).unwrap() {
            Some((left, right)) => {
                stones.push(Stone(left));
                stones.push(Stone(right));
            }
            None => stones.push(Stone(value * 2024)),
        },
    }

    stones
}

#[derive(Debug, PartialEq)]
pub struct ParseU64Error(pub String, pub ParseIntError);

pub fn split_on_two(value: u64) -> Result<Option<(u64, u64)>, ParseU64Error> {
    let s = value.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return Ok(None);
    }

    let (left, right) = s.split_at(len / 2);

    let left: u64 = left
        .parse()
        .map_err(|err| ParseU64Error(s.to_owned(), err))?;
    let right: u64 = right
        .parse()
        .map_err(|err| ParseU64Error(s.to_owned(), err))?;

    Ok(Some((left, right)))
}

#[cfg(test)]
mod tests {
    use crate::{split_stones, Stone};

    #[test]
    fn split_on_two() {
        assert_eq!(Ok(Some((253, 0))), super::split_on_two(253000));
        assert_eq!(Ok(None), super::split_on_two(253));
    }

    #[test]
    fn split_example() {
        let stones_initial = super::parse_input("125 17").unwrap();

        let stones_1 = stones_initial;
        assert_eq!(
            vec![Stone(253000), Stone(1), Stone(7)],
            split_stones(&stones_1)
        );

        let stones_2 = split_stones(&stones_1);
        assert_eq!(
            vec![Stone(253), Stone(0), Stone(2024), Stone(14168)],
            split_stones(&stones_2)
        );

        let stones_3 = split_stones(&stones_2);
        assert_eq!(
            vec![
                Stone(512072),
                Stone(1),
                Stone(20),
                Stone(24),
                Stone(28676032)
            ],
            split_stones(&stones_3)
        );

        let stones_4 = split_stones(&stones_3);
        assert_eq!(
            vec![
                Stone(512),
                Stone(72),
                Stone(2024),
                Stone(2),
                Stone(0),
                Stone(2),
                Stone(4),
                Stone(2867),
                Stone(6032)
            ],
            split_stones(&stones_4)
        );

        let stones_5 = split_stones(&stones_4);
        assert_eq!(
            vec![
                Stone(1036288),
                Stone(7),
                Stone(2),
                Stone(20),
                Stone(24),
                Stone(4048),
                Stone(1),
                Stone(4048),
                Stone(8096),
                Stone(28),
                Stone(67),
                Stone(60),
                Stone(32)
            ],
            split_stones(&stones_5)
        );

        let stones_6 = split_stones(&stones_5);
        assert_eq!(22, split_stones(&stones_6).len())
    }
}
