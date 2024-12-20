use std::{collections::HashMap, num::ParseIntError};

fn main() {
    let input = "4 4841539 66 5279 49207 134 609568 0";

    println!("Part 1: {}", part1(input, 25));
    println!("Part 2: {}", part2(input, 75));
}

pub fn part1(input: &str, blinks: usize) -> usize {
    let mut stones = parse_input(input).unwrap();

    for _ in 0..blinks {
        stones = split_stones(&stones)
    }

    stones.len()
}

pub fn part2(input: &str, blinks: usize) -> usize {
    let mut occurences = Occurences::from_input(input).unwrap();

    for _ in 0..blinks {
        occurences = part2_split_stones(occurences);
    }

    occurences.0.values().sum()
}

pub fn part2_split_stones(occurences: Occurences) -> Occurences {
    let mut out = occurences.clone();

    occurences
        .0
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .for_each(|(stone, count)| {
            *out.0.entry(stone).or_default() -= count;

            split_stone(stone)
                .into_iter()
                .for_each(|stone| *out.0.entry(stone).or_default() += count);
        });

    out
}

#[derive(Debug, Clone, PartialEq)]
pub struct Occurences(pub HashMap<Stone, usize>);
impl Occurences {
    pub fn from_input(input: &str) -> Result<Occurences, ParseU64Error> {
        let stones = parse_input(input)?;
        Ok(Occurences::from_slice(&stones))
    }

    pub fn from_slice(stones: &[Stone]) -> Occurences {
        let mut map: HashMap<Stone, usize> = HashMap::new();
        stones
            .iter()
            .for_each(|stone| *map.entry(*stone).or_default() += 1);

        Occurences(map)
    }

    pub fn remove_empty(self) -> Occurences {
        Occurences(self.0.into_iter().filter(|(_, n)| *n > 0).collect())
    }
}

impl std::fmt::Display for Occurences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries = self.0.clone().into_iter().collect::<Vec<(Stone, usize)>>();
        entries.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
        for (stone, n) in entries {
            writeln!(f, "{} => {}", stone.0, n)?;
        }

        Ok(())
    }
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

    #[test]
    fn part2_stones_equals_part1() {
        let input = "4 4841539 66 5279 49207 134 609568 0";
        assert_eq!(super::part1(input, 25), super::part2(input, 25));
    }
}
