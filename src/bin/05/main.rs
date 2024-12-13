use std::num::ParseIntError;

fn main() {
    let data = read_input(&std::fs::read_to_string("./files/05.txt").unwrap()).unwrap();

    println!("Day 5");

    println!("Part 1: {}", part1(&data))
}

#[derive(Debug, Clone, Copy)]
pub struct Rule(pub u32, pub u32);

#[derive(Debug, Clone)]
pub struct Update(pub Vec<u32>);

#[derive(Debug, Default)]
pub struct Data {
    pub rules: Vec<Rule>,
    pub updates: Vec<Update>,
}

#[derive(Debug)]
pub enum ParseDataError {
    Rule(ParseIntError, String),
    Update(ParseIntError, String),
}

pub fn part1(data: &Data) -> u32 {
    data.updates
        .iter()
        .map(|update| {
            let is_correct = is_update_correct(update, &data.rules);
            if !is_correct {
                return 0;
            };

            find_middle(&update.0).unwrap_or_default()
        })
        .sum()
}

pub fn is_update_correct(update: &Update, rules: &[Rule]) -> bool {
    let vec: Vec<u32> = update
        .0
        .iter()
        .enumerate()
        .filter(|(index, value)| {
            let left = value;
            let Some(right) = update.0.get(index + 1) else {
                return true;
            };

            rules
                .iter()
                .any(|rule| rule.0 == **left && rule.1 == *right)
        })
        .map(|(_, v)| v)
        .copied()
        .collect();

    if vec.len() != update.0.len() {
        return false;
    }

    true
}

pub fn find_middle(numbers: &[u32]) -> Option<u32> {
    if numbers.len() % 2 == 0 {
        return None;
    };

    Some(numbers[numbers.len() / 2])
}

pub fn read_input(s: &str) -> Result<Data, ParseDataError> {
    let mut data = Data::default();

    for line in s.lines() {
        if line.contains("|") {
            let (left, right) = line.split_once("|").unwrap();
            let left = left
                .parse::<u32>()
                .map_err(|err| ParseDataError::Rule(err, left.to_owned()))?;
            let right = right
                .parse::<u32>()
                .map_err(|err| ParseDataError::Rule(err, right.to_owned()))?;
            data.rules.push(Rule(left, right));
        } else if line.contains(",") {
            let update_nums = line
                .split(",")
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|err| ParseDataError::Update(err, s.to_owned()))
                })
                .collect::<Result<Vec<u32>, ParseDataError>>()?;
            data.updates.push(Update(update_nums));
        }
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    const INPUT_EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn read_input() {
        let data = super::read_input(INPUT_EXAMPLE).unwrap();
        assert_eq!(21, data.rules.len());
        assert_eq!(6, data.updates.len());
    }

    #[test]
    fn is_update_correct() {
        let data = super::read_input(INPUT_EXAMPLE).unwrap();
        let updates = &data.updates;
        let rules = &data.rules;

        assert!(super::is_update_correct(&updates[0], rules));
        assert!(super::is_update_correct(&updates[1], rules));
        assert!(super::is_update_correct(&updates[2], rules));
        assert!(!super::is_update_correct(&updates[3], rules));
        assert!(!super::is_update_correct(&updates[4], rules));
        assert!(!super::is_update_correct(&updates[5], rules));
    }

    #[test]
    fn find_middle() {
        assert_eq!(super::find_middle(&[75, 47, 61, 53, 29]), Some(61));
        assert_eq!(super::find_middle(&[75, 47, 53, 29]), None);
    }

    #[test]
    fn part1() {
        let data = super::read_input(INPUT_EXAMPLE).unwrap();
        assert_eq!(super::part1(&data), 143);
    }
}
