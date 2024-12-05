use std::str::Chars;

fn main() {
    let input = std::fs::read_to_string("./files/03.txt").unwrap();

    println!("Part 1 sum: {}", calc_input_muls(&input));
}

pub fn calc_mul(mul: &Mul) -> u32 {
    mul.0 * mul.1
}

pub fn calc_input_muls(input: &str) -> u32 {
    let muls = find_correct_muls(input);
    muls.iter().map(calc_mul).sum()
}

#[derive(Debug, PartialEq)]
pub struct Mul(pub u32, pub u32);

pub fn find_correct_muls(input: &str) -> Vec<Mul> {
    let mut muls: Vec<Mul> = vec![];

    let mut chars = input.chars();

    while let Some(mul) = find_next_mul(&mut chars) {
        muls.push(mul);
    }

    muls
}

#[allow(clippy::while_let_on_iterator)]
fn find_next_mul(chars: &mut Chars) -> Option<Mul> {
    let mut buffer = String::new();

    while let Some(ch) = chars.next() {
        buffer.push(ch);

        if buffer.len() == 4 {
            if buffer.as_str() == "mul(" {
                let mut first_number_buffer = String::new();

                'find_first_number: while let Some(ch) = chars.next() {
                    match ch {
                        ch if ch.is_ascii_digit() => first_number_buffer.push(ch),
                        ',' => {
                            let mut second_number_buffer = String::new();
                            while let Some(ch) = chars.next() {
                                match ch {
                                    ch if ch.is_ascii_digit() => second_number_buffer.push(ch),
                                    ')' => {
                                        return Some(Mul(
                                            first_number_buffer.parse().unwrap(),
                                            second_number_buffer.parse().unwrap(),
                                        ))
                                    }
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
    use crate::Mul;

    #[test]
    fn find_correct_muls() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            super::find_correct_muls(s),
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );
    }

    #[test]
    fn calc_input_muls() {
        assert_eq!(
            161,
            super::calc_input_muls(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )
        )
    }
}
