use crate::part2::operation::Operation;
use std::{num::ParseIntError, str::FromStr};

pub fn parse_equations(input: &str) -> Result<Vec<Equation>, ParseEquationError> {
    input.lines().map(|s| s.parse()).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    pub test_value: u64,
    pub values: Vec<u64>,
}

impl Equation {
    pub fn is_possible(&self) -> bool {
        crate::part2::operation::generate_combinations(self.values.len() - 1)
            .iter()
            .any(|operations| self.test_value == self.eval(operations))
    }

    pub fn eval(&self, operations: &[Operation]) -> u64 {
        let Some(mut result) = self.values.first().copied() else {
            return 0;
        };

        for (i, op) in operations.iter().enumerate() {
            let Some(next_number) = self.values.get(i + 1) else {
                break;
            };

            match op {
                Operation::Add => result += next_number,
                Operation::Mul => result *= next_number,
                Operation::Concat => {
                    result = format!("{result}{next_number}").parse().unwrap();
                }
            }
        }

        result
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum ParseEquationError {
    NoColon(String),
    ParseInt(ParseIntError, String),
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((test_value, values_str)) = s.split_once(":") else {
            return Err(ParseEquationError::NoColon(s.to_owned()));
        };

        let test_value = test_value
            .parse::<u64>()
            .map_err(|err| ParseEquationError::ParseInt(err, test_value.to_owned()))?;

        let values = values_str
            .trim()
            .split(" ")
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|err| ParseEquationError::ParseInt(err, s.to_owned()))
            })
            .collect::<Result<Vec<u64>, ParseEquationError>>()?;

        Ok(Equation { test_value, values })
    }
}

#[cfg(test)]
mod tests {
    use super::Equation;

    #[test]
    fn is_possible() {
        let p = |s: &str| -> Equation { s.parse().unwrap() };

        assert!(p("190: 10 19").is_possible());
        assert!(p("3267: 81 40 27").is_possible());
        assert!(!p("83: 17 5").is_possible());
        assert!(p("156: 15 6").is_possible());
        assert!(p("7290: 6 8 6 15").is_possible());
        assert!(!p("161011: 16 10 13").is_possible());
        assert!(p("192: 17 8 14").is_possible());
        assert!(p("292: 11 6 16 20").is_possible());
    }

    #[test]
    fn parse() {
        assert_eq!(
            Equation {
                test_value: 190,
                values: vec![10, 19]
            },
            "190: 10 19".parse::<Equation>().unwrap()
        );
    }
}
