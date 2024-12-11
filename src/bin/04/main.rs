use std::fmt::Write;

fn main() {
    println!("Advent of code day 4");

    let board = Board(parse_input(&std::fs::read_to_string("./files/04.txt").unwrap()).unwrap());
    println!("Part 1. {}", count_xmas(&board));
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    pub fn as_char(&self) -> char {
        match self {
            Letter::X => 'X',
            Letter::M => 'M',
            Letter::A => 'A',
            Letter::S => 'S',
        }
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[derive(Debug)]
pub struct IncorrectLetterError(pub char);

pub fn parse_input(input: &str) -> Result<Vec<Vec<Letter>>, IncorrectLetterError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => Ok(Letter::X),
                    'M' => Ok(Letter::M),
                    'A' => Ok(Letter::A),
                    'S' => Ok(Letter::S),
                    c => Err(IncorrectLetterError(c)),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
pub struct Board(pub Vec<Vec<Letter>>);

pub fn count_xmas(board: &Board) -> u32 {
    let mut n = 0;
    let directions = Direction::directions();

    for (i, row) in board.0.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter != Letter::X {
                continue;
            }

            for direction in directions {
                let mut stepper = Stepper {
                    i,
                    j,
                    board: &board.0,
                };

                if let Some(Letter::M) = stepper.step(direction) {
                    if let Some(Letter::A) = stepper.step(direction) {
                        if let Some(Letter::S) = stepper.step(direction) {
                            n += 1;
                        }
                    }
                }
            }
        }
    }

    n
}

#[derive(Debug, Clone)]
pub struct Stepper<'board, T> {
    pub i: usize,
    pub j: usize,
    pub board: &'board [Vec<T>],
}

impl<T: Copy> Stepper<'_, T> {
    pub fn step(&mut self, direction: Direction) -> Option<T>
    where
        T: Copy,
    {
        let m = self.board;
        let i = &mut self.i;
        let j = &mut self.j;

        match direction {
            Direction::Left => {
                if *j == 0 {
                    return None;
                }
                *j -= 1
            }
            Direction::Up => {
                if *i == 0 {
                    return None;
                }
                *i -= 1
            }
            Direction::Right => *j += 1,
            Direction::Down => *i += 1,
            Direction::UpLeft => {
                if *i == 0 || *j == 0 {
                    return None;
                }

                *i -= 1;
                *j -= 1;
            }
            Direction::UpRight => {
                if *i == 0 {
                    return None;
                }
                *i -= 1;
                *j += 1;
            }
            Direction::DownLeft => {
                if *j == 0 {
                    return None;
                }

                *i += 1;
                *j -= 1;
            }
            Direction::DownRight => {
                *i += 1;
                *j += 1;
            }
        };

        m.get(*i).and_then(|r| r.get(*j)).copied()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub const fn directions() -> [Direction; 8] {
        [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Direction, Letter, Stepper};

    #[test]
    fn count_xmas() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let board = Board(super::parse_input(input).unwrap());

        assert_eq!(super::count_xmas(&board), 18);
    }

    #[test]
    fn stepper() {
        let input = "MMMSXXMASM\nMSAMXMSMSA";
        let m = super::parse_input(input).unwrap();

        let mut stepper = Stepper {
            i: 1,
            j: 0,
            board: &m,
        };

        assert_eq!(stepper.step(Direction::Right), Some(Letter::S));
        assert_eq!(stepper.step(Direction::Right), Some(Letter::A));
        assert_eq!(stepper.step(Direction::Right), Some(Letter::M));
        assert_eq!(stepper.step(Direction::Up), Some(Letter::S));
        assert_eq!(stepper.step(Direction::Left), Some(Letter::M));
        assert_eq!(stepper.step(Direction::UpRight), None);
        assert_eq!(stepper.step(Direction::DownRight), Some(Letter::M));
        assert_eq!(stepper.step(Direction::UpRight), Some(Letter::X));
    }

    #[test]
    fn parse_input() {
        let input = "MMMSXXMASM\nMSAMXMSMSA";

        assert_eq!(
            super::parse_input(input).unwrap(),
            vec![
                vec![
                    Letter::M,
                    Letter::M,
                    Letter::M,
                    Letter::S,
                    Letter::X,
                    Letter::X,
                    Letter::M,
                    Letter::A,
                    Letter::S,
                    Letter::M
                ],
                vec![
                    Letter::M,
                    Letter::S,
                    Letter::A,
                    Letter::M,
                    Letter::X,
                    Letter::M,
                    Letter::S,
                    Letter::M,
                    Letter::S,
                    Letter::A
                ]
            ]
        );
    }
}
