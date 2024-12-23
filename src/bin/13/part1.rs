use std::{cmp, num::ParseIntError, str::FromStr};

pub fn parse_input(input: &str) -> Result<Vec<Group>, ParseGroupError> {
    input.split("\n\n").map(|s| s.parse()).collect()
}

#[derive(Debug, Clone)]
pub struct Group {
    pub a: ButtonA,
    pub b: ButtonB,
    pub prize: Prize,
}

impl Group {
    pub fn find_solution(&self) -> Option<usize> {
        let a = self.a;
        let b = self.b;
        let prize = self.prize;

        let get_b_solution = || {
            let div_x = prize.x.0.div_ceil(b.x.0);
            let div_y = prize.y.0.div_ceil(b.y.0);
            let mut b_count = cmp::min(div_x, div_y);
            let mut a_count: usize = 0;

            loop {
                if sum_equals_prize(a, b, prize, a_count, b_count) {
                    return Some(a_count * a.token_cost() + b_count * b.token_cost());
                }
                b_count -= 1;

                loop {
                    //

                    if sum_equals_prize(a, b, prize, a_count, b_count) {
                        return Some(a_count * a.token_cost() + b_count * b.token_cost());
                    }

                    let sum = sum(a, b, a_count, b_count);
                    if sum.0 > prize.x || sum.1 > prize.y {
                        break;
                    }
                    a_count += 1;
                }

                if b_count == 0 {
                    return None;
                }
            }
        };

        let get_a_solution = || {
            // let div_x = prize.x / a.x

            let div_x = prize.x.0.div_ceil(a.x.0);
            let div_y = prize.y.0.div_ceil(a.y.0);
            let mut b_count: usize = 0;
            let mut a_count = cmp::min(div_x, div_y);

            loop {
                if sum_equals_prize(a, b, prize, a_count, b_count) {
                    return Some(a_count * a.token_cost() + b_count * b.token_cost());
                }
                a_count -= 1;

                loop {
                    if sum_equals_prize(a, b, prize, a_count, b_count) {
                        return Some(a_count * a.token_cost() + b_count * b.token_cost());
                    }

                    let sum = sum(a, b, a_count, b_count);
                    if sum.0 > prize.x || sum.1 > prize.y {
                        break;
                    }
                    b_count += 1;
                }

                if a_count == 0 {
                    return None;
                }
            }
        };

        let a_solution = get_a_solution();
        let b_solution = get_b_solution();

        if a_solution.is_none() && b_solution.is_none() {
            return None;
        };

        let a_solution = a_solution.unwrap_or_default();
        let b_solution = b_solution.unwrap_or_default();

        Some(cmp::min(a_solution, b_solution))
    }
}

#[derive(Debug)]
pub enum ParseGroupError {
    ShouldHaveThreeLines,
    A(ParseButtonAError),
    B(ParseButtonBError),
    Prize(ParsePrizeError),
}

impl FromStr for Group {
    type Err = ParseGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a: ButtonA = lines
            .next()
            .ok_or(ParseGroupError::ShouldHaveThreeLines)?
            .parse()
            .map_err(ParseGroupError::A)?;

        let b: ButtonB = lines
            .next()
            .ok_or(ParseGroupError::ShouldHaveThreeLines)?
            .parse()
            .map_err(ParseGroupError::B)?;

        let prize: Prize = lines
            .next()
            .ok_or(ParseGroupError::ShouldHaveThreeLines)?
            .parse()
            .map_err(ParseGroupError::Prize)?;

        Ok(Group { a, b, prize })
    }
}

fn sum(a: ButtonA, b: ButtonB, a_count: usize, b_count: usize) -> (X, Y) {
    (
        X(a_count * a.x.0 + b_count * b.x.0),
        Y(a_count * a.y.0 + b_count * b.y.0),
    )
}

fn sum_equals_prize(a: ButtonA, b: ButtonB, prize: Prize, a_count: usize, b_count: usize) -> bool {
    let sum = sum(a, b, a_count, b_count);
    sum.0 == prize.x && sum.1 == prize.y
}

trait TokenCost {
    fn token_cost(&self) -> usize;
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonA {
    pub x: X,
    pub y: Y,
}

#[derive(Debug, Clone)]
pub enum ParseButtonAError {
    NoColonDelimiter,
    ShouldStartWithButtonA(String),
    MissingCommaExpressionsDelimeter(String),
    InvalidXExpression(String),
    InvalidYExpression(String),
    ParseInt(String, ParseIntError),
}

impl FromStr for ButtonA {
    type Err = ParseButtonAError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(":")
            .ok_or(ParseButtonAError::NoColonDelimiter)?;
        if left != "Button A" {
            return Err(ParseButtonAError::ShouldStartWithButtonA(left.to_owned()));
        }

        let (left_expr, right_expr) = right.trim().split_once(",").ok_or(
            ParseButtonAError::MissingCommaExpressionsDelimeter(right.to_owned()),
        )?;
        let left_expr = left_expr.trim();
        let right_expr = right_expr.trim();

        if !left_expr.starts_with("X+") {
            return Err(ParseButtonAError::InvalidXExpression(left_expr.to_owned()));
        };

        let x_value = left_expr[2..]
            .parse::<usize>()
            .map_err(|err| ParseButtonAError::ParseInt(left_expr[2..].to_owned(), err))?;

        if !right_expr.starts_with("Y+") {
            return Err(ParseButtonAError::InvalidYExpression(right_expr.to_owned()));
        };

        let y_value = right_expr[2..]
            .trim()
            .parse::<usize>()
            .map_err(|err| ParseButtonAError::ParseInt(right_expr[2..].to_owned(), err))?;

        Ok(ButtonA {
            x: X(x_value),
            y: Y(y_value),
        })
    }
}

impl TokenCost for ButtonA {
    fn token_cost(&self) -> usize {
        3
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonB {
    pub x: X,
    pub y: Y,
}

#[derive(Debug, Clone)]
pub enum ParseButtonBError {
    NoColonDelimiter,
    ShouldStartWithButtonB(String),
    MissingCommaExpressionsDelimeter(String),
    InvalidXExpression(String),
    InvalidYExpression(String),
    ParseInt(String, ParseIntError),
}

impl FromStr for ButtonB {
    type Err = ParseButtonBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(":").ok_or(Self::Err::NoColonDelimiter)?;
        let left = left.trim();
        let right = right.trim();
        if left != "Button B" {
            return Err(Self::Err::ShouldStartWithButtonB(left.to_owned()));
        }

        let (left_expr, right_expr) =
            right
                .trim()
                .split_once(",")
                .ok_or(Self::Err::MissingCommaExpressionsDelimeter(
                    right.to_owned(),
                ))?;

        let left_expr = left_expr.trim();
        let right_expr = right_expr.trim();

        if !left_expr.starts_with("X+") {
            return Err(Self::Err::InvalidXExpression(left_expr.to_owned()));
        };

        let x_value = left_expr[2..]
            .trim()
            .parse::<usize>()
            .map_err(|err| Self::Err::ParseInt(left_expr[2..].to_owned(), err))?;

        if !right_expr.starts_with("Y+") {
            return Err(Self::Err::InvalidYExpression(right_expr.to_owned()));
        };

        let y_value = right_expr[2..]
            .trim()
            .parse::<usize>()
            .map_err(|err| Self::Err::ParseInt(right_expr[2..].to_owned(), err))?;

        Ok(ButtonB {
            x: X(x_value),
            y: Y(y_value),
        })
    }
}

impl TokenCost for ButtonB {
    fn token_cost(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Prize {
    pub x: X,
    pub y: Y,
}

#[derive(Debug)]
pub enum ParsePrizeError {
    NoColonDelimiter(String),
    ShouldStartWithPrizeWord(String),
    MissingCommaExpressionsDelimeter(String),
    InvalidXExpression(String),
    InvalidYExpression(String),
    ParseInt(String, ParseIntError),
}

impl FromStr for Prize {
    type Err = ParsePrizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prize_word, expressions) = s
            .split_once(":")
            .ok_or(ParsePrizeError::NoColonDelimiter(s.to_owned()))?;
        let prize_word = prize_word.trim();
        let expressions = expressions.trim();

        if prize_word != "Prize" {
            return Err(ParsePrizeError::ShouldStartWithPrizeWord(
                prize_word.to_owned(),
            ));
        }

        let (x_expr, y_expr) = expressions.split_once(",").ok_or(
            ParsePrizeError::MissingCommaExpressionsDelimeter(expressions.to_owned()),
        )?;
        let x_expr = x_expr.trim();
        let y_expr = y_expr.trim();

        if !x_expr.starts_with("X=") {
            return Err(ParsePrizeError::InvalidXExpression(x_expr.to_owned()));
        };

        let x_value = x_expr[2..]
            .parse::<usize>()
            .map_err(|err| ParsePrizeError::ParseInt(x_expr.to_owned(), err))?;

        if !y_expr.starts_with("Y=") {
            return Err(ParsePrizeError::InvalidYExpression(y_expr.to_owned()));
        };

        let y_value = y_expr[2..]
            .parse::<usize>()
            .map_err(|err| ParsePrizeError::ParseInt(y_expr.to_owned(), err))?;

        Ok(Prize {
            x: X(x_value),
            y: Y(y_value),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct X(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Y(pub usize);
