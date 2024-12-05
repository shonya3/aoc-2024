use std::str::Chars;

pub fn calc_mul(mul: &Mul) -> u32 {
    mul.0 * mul.1
}

pub fn calc_input_muls(input: &str) -> u32 {
    let muls = find_correct_muls(input);
    muls.iter().map(calc_mul).sum()
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Command {
    #[default]
    Do,
    Dont,
}

pub enum FindMul {
    Mul(Mul),
    Skip,
}

#[derive(Debug, PartialEq)]
pub struct Mul(pub u32, pub u32);

pub fn find_correct_muls(input: &str) -> Vec<Mul> {
    let mut muls: Vec<Mul> = vec![];

    let mut chars = input.chars();
    let mut command = Command::default();

    while let Some(mul) = find_next_mul(&mut chars, &mut command) {
        if let FindMul::Mul(mul) = mul {
            muls.push(mul);
        }
    }

    muls
}

#[allow(clippy::while_let_on_iterator)]
fn find_next_mul(chars: &mut Chars, command: &mut Command) -> Option<FindMul> {
    let mut buffer = String::new();
    let mut command_buffer = String::new();

    while let Some(ch) = chars.next() {
        buffer.push(ch);
        command_buffer.push(ch);

        if command_buffer.contains("do()") {
            *command = Command::Do;
        }

        if command_buffer.contains("don't()") {
            *command = Command::Dont;
        }

        if buffer.len() == 4 {
            if buffer.as_str() == "mul(" {
                let mut first_number_buffer = String::new();
                command_buffer.clear();

                'find_first_number: while let Some(ch) = chars.next() {
                    match ch {
                        ch if ch.is_ascii_digit() => first_number_buffer.push(ch),
                        ',' => {
                            let mut second_number_buffer = String::new();
                            while let Some(ch) = chars.next() {
                                match ch {
                                    ch if ch.is_ascii_digit() => second_number_buffer.push(ch),
                                    ')' => match command {
                                        Command::Do => {
                                            let mul = Mul(
                                                first_number_buffer.parse().unwrap(),
                                                second_number_buffer.parse().unwrap(),
                                            );
                                            return Some(FindMul::Mul(mul));
                                        }
                                        Command::Dont => return Some(FindMul::Skip),
                                    },
                                    _ => {
                                        buffer.clear();
                                        break 'find_first_number;
                                    }
                                }
                            }
                        }
                        _ => {
                            buffer.clear();
                            break 'find_first_number;
                        }
                    }
                }
            } else {
                buffer.remove(0);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::part_2::Mul;

    #[test]
    fn find_correct_muls() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(super::find_correct_muls(s), vec![Mul(2, 4), Mul(8, 5)]);
    }
}
