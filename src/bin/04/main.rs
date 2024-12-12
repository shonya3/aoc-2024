use std::fmt::Write;

fn main() {
    println!("Advent of code day 4");

    let board = Board(parse_input(&std::fs::read_to_string("./files/04.txt").unwrap()).unwrap());
    println!("Part 1. {}", count_xmas(&board));
    println!("Part 2. {}", count_xmas_crosses(&board));
}

pub fn count_xmas(board: &Board<Letter>) -> u32 {
    let mut n = 0;
    let directions = Direction::directions();

    for (i, row) in board.0.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter != Letter::X {
                continue;
            }

            for direction in directions {
                let mut stepper = Stepper { i, j, board };

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

pub fn count_xmas_crosses(board: &Board<Letter>) -> u32 {
    let check_cross = |letter: &Letter, stepper: &Stepper<Letter>| -> bool {
        if *letter == Letter::X || *letter == Letter::A {
            return false;
        }

        let Some(Cross {
            top_l,
            top_r,
            bot_l,
            bot_r,
            mid,
        }) = stepper.cross()
        else {
            return false;
        };

        if mid != Letter::A {
            return false;
        }

        use Letter::{M, S};

        let v1 = top_l == bot_l && top_l == M && top_r == bot_r && top_r == S;
        let v2 = top_l == bot_l && top_l == S && top_r == bot_r && top_r == M;
        let v3 = top_l == top_r && top_l == S && bot_l == bot_r && bot_r == M;
        let v4 = top_l == top_r && top_l == M && bot_l == bot_r && bot_r == S;

        v1 || v2 || v3 || v4
    };

    let mut n = 0;
    for (i, row) in board.0.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            let stepper = Stepper { i, j, board };

            if check_cross(letter, &stepper) {
                n += 1;
            };
        }
    }

    n
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
pub struct Board<T>(pub Vec<Vec<T>>);

impl<T: Copy> Board<T> {
    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        self.0.get(i).and_then(|row| row.get(j)).copied()
    }
}

#[derive(Debug, Clone)]
pub struct Stepper<'board, T> {
    pub i: usize,
    pub j: usize,
    pub board: &'board Board<T>,
}

impl<T: Copy> Stepper<'_, T> {
    pub fn step(&mut self, direction: Direction) -> Option<T>
    where
        T: Copy,
    {
        let (next_i, next_j) = next_i_j(self.i, self.j, direction)?;

        self.i = next_i;
        self.j = next_j;

        self.board.get(next_i, next_j)
    }

    pub fn cross(&self) -> Option<Cross<T>> {
        let i = self.i;
        let j = self.j;
        Some(Cross {
            top_l: self.board.get(i, j)?,
            top_r: self.board.get(i, j + 2)?,
            bot_l: self.board.get(i + 2, j)?,
            bot_r: self.board.get(i + 2, j + 2)?,
            mid: self.board.get(i + 1, j + 1)?,
        })
    }
}

/// 3x3 "X"
pub struct Cross<T> {
    top_l: T,
    top_r: T,
    bot_l: T,
    bot_r: T,
    mid: T,
}

pub fn next_i_j(i: usize, j: usize, direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Left => match j == 0 {
            true => None,
            false => Some((i, j - 1)),
        },
        Direction::Up => match i == 0 {
            true => None,
            false => Some((i - 1, j)),
        },
        Direction::Right => Some((i, j + 1)),
        Direction::Down => Some((i + 1, j)),
        Direction::UpLeft => {
            if i == 0 || j == 0 {
                None
            } else {
                Some((i - 1, j - 1))
            }
        }
        Direction::UpRight => match i == 0 {
            true => None,
            false => Some((i - 1, j + 1)),
        },
        Direction::DownLeft => match j == 0 {
            true => None,
            false => Some((i + 1, j - 1)),
        },
        Direction::DownRight => Some((i + 1, j + 1)),
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
    fn count_xmas_crosses() {
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

        assert_eq!(9, super::count_xmas_crosses(&board));
    }

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
        let board = Board(super::parse_input(input).unwrap());

        let mut stepper = Stepper {
            i: 1,
            j: 0,
            board: &board,
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
