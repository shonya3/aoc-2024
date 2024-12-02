use std::{collections::HashMap, num::ParseIntError};

fn main() {
    let s = std::fs::read_to_string("./files/01/lists.txt").unwrap();
    let (l1, l2) = read_input(&s).unwrap();

    println!("PART 1. Distance of two lists is: {}", distance(&l1, &l2));
    println!(
        "PART 2. Similarity score of two lists is: {}",
        similarity_score(&l1, &l2)
    );
}

pub fn distance(list_1: &[u32], list_2: &[u32]) -> u32 {
    let mut l1 = Vec::from_iter(list_1);
    let mut l2 = Vec::from_iter(list_2);

    l1.sort();
    l2.sort();

    l1.iter().zip(l2).map(|(a, b)| a.abs_diff(*b)).sum()
}

pub fn similarity_score(list_1: &[u32], list_2: &[u32]) -> u32 {
    let mut list_2_occurences_map: HashMap<u32, u32> = HashMap::new();
    list_2
        .iter()
        .for_each(|n| *list_2_occurences_map.entry(*n).or_default() += 1);

    list_1
        .iter()
        .map(|n| n * list_2_occurences_map.get(n).copied().unwrap_or_default())
        .sum()
}

#[derive(Debug)]
pub enum ReadInputError {
    SplitLine(String),
    ParseU32(ParseIntError, String),
}

/// Collect two lists into to vecs. Lists example:
/// 3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3
pub fn read_input(input: &str) -> Result<(Vec<u32>, Vec<u32>), ReadInputError> {
    input
        .lines()
        .map(|s| {
            let mut split = s.split("   ");
            let a = split
                .next()
                .ok_or_else(|| ReadInputError::SplitLine(s.to_owned()))?;
            let a = a
                .parse::<u32>()
                .map_err(|err| ReadInputError::ParseU32(err, a.to_owned()))?;

            let b = split
                .next()
                .ok_or_else(|| ReadInputError::SplitLine(s.to_owned()))?;
            let b = b
                .parse::<u32>()
                .map_err(|err| ReadInputError::ParseU32(err, b.to_owned()))?;

            Ok((a, b))
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn distance() {
        let list_1 = &[3, 4, 2, 1, 3, 3];
        let list_2 = &[4, 3, 5, 3, 9, 3];
        assert_eq!(super::distance(list_1, list_2), 11);
    }

    #[test]
    fn similarity_score() {
        let list_1 = &[3, 4, 2, 1, 3, 3];
        let list_2 = &[4, 3, 5, 3, 9, 3];
        assert_eq!(super::similarity_score(list_1, list_2), 31);
    }

    #[test]
    fn read_input() {
        let res = super::read_input("3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
        assert_eq!(
            res.unwrap(),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }
}
