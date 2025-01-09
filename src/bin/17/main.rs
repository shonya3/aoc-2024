use std::{num::ParseIntError, str::FromStr};

pub const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

fn main() {
    println!("Day 17");
}

#[derive(Debug, Clone, PartialEq)]
pub struct Computer {
    pub a: RegisterA,
    pub b: RegisterB,
    pub c: RegisterC,
    pub program: Program,
}

#[derive(Debug)]
pub enum ParseComputerError {
    ParseRegister(char, ParseRegisterError),
    ParseProgram(ParseProgramError),
    InvalidInput(String),
}

impl FromStr for Computer {
    type Err = ParseComputerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut take_line = || {
            lines
                .next()
                .ok_or_else(|| ParseComputerError::InvalidInput(s.to_owned()))
        };

        let a_input = take_line()?;
        let b_input = take_line()?;
        let c_input = take_line()?;
        let empty = take_line()?;
        if !empty.is_empty() {
            return Err(ParseComputerError::InvalidInput(s.to_owned()));
        }
        let program_input = take_line()?;

        let a: RegisterA = a_input
            .parse()
            .map_err(|err| ParseComputerError::ParseRegister('A', err))?;
        let b: RegisterB = b_input
            .parse()
            .map_err(|err| ParseComputerError::ParseRegister('B', err))?;
        let c: RegisterC = c_input
            .parse()
            .map_err(|err| ParseComputerError::ParseRegister('C', err))?;

        let program: Program = program_input
            .parse()
            .map_err(ParseComputerError::ParseProgram)?;
        Ok(Computer { a, b, c, program })
    }
}

#[derive(Debug)]
pub enum ParseRegisterError {
    InvalidInput,
    ParseInt(ParseIntError, String),
}

pub fn parse_register(input: &str, ch: char) -> Result<u32, ParseRegisterError> {
    let start = format!("Register {ch}:");
    if !input.starts_with(&start) {
        return Err(ParseRegisterError::InvalidInput);
    }

    let s = input.replace(&start, "");
    let s = s.trim();
    let value = s
        .parse::<u32>()
        .map_err(|err| ParseRegisterError::ParseInt(err, s.to_owned()))?;
    Ok(value)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterA(pub u32);

impl FromStr for RegisterA {
    type Err = ParseRegisterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegisterA(parse_register(s, 'A')?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterB(pub u32);

impl FromStr for RegisterB {
    type Err = ParseRegisterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegisterB(parse_register(s, 'B')?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegisterC(pub u32);

impl FromStr for RegisterC {
    type Err = ParseRegisterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegisterC(parse_register(s, 'C')?))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand(pub u8);

#[derive(Debug)]
pub enum ParseProgramError {
    InvalidInput(String),
    ParseInt(ParseIntError, String),
    ParseInstruction(InstructionCodeShouldBeBetweenZeroAndSeven),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<(Instruction, Operand)>);

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expected_start = "Program:";
        if !s.starts_with(expected_start) {
            return Err(ParseProgramError::InvalidInput(s.to_owned()));
        }

        let s = s.replace(expected_start, "");
        let s = s.trim();
        let pairs = s
            .split(',')
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|err| ParseProgramError::ParseInt(err, s.to_owned()))
            })
            .collect::<Result<Vec<u8>, ParseProgramError>>()?
            .chunks_exact(2)
            .map(|chunk| Ok((Instruction::try_from(chunk[0])?, Operand(chunk[1]))))
            .collect::<Result<Vec<(Instruction, Operand)>, InstructionCodeShouldBeBetweenZeroAndSeven>>(
            )
            .map_err(ParseProgramError::ParseInstruction)?;

        Ok(Program(pairs))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    /// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
    /// The result of the division operation is truncated to an integer and then written to the A register.
    Adv,
    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
    /// then stores the result in register B.
    Bxl,
    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst,
    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
    /// if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    Jnz,
    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    Bxc,
    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
    /// (If a program outputs multiple values, they are separated by commas.)
    Out,
    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
    /// (The numerator is still read from the A register.)
    Bdv,
    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
    /// (The numerator is still read from the A register.)
    Cdv,
}

#[derive(Debug)]
pub struct InstructionCodeShouldBeBetweenZeroAndSeven(pub u8);

impl TryFrom<u8> for Instruction {
    type Error = InstructionCodeShouldBeBetweenZeroAndSeven;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(InstructionCodeShouldBeBetweenZeroAndSeven(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Computer, Instruction, Operand, Program, RegisterA, RegisterB, RegisterC, EXAMPLE,
    };

    #[test]
    fn parse() {
        let computer: Computer = EXAMPLE.parse().unwrap();
        assert_eq!(
            Computer {
                a: RegisterA(729),
                b: RegisterB(0),
                c: RegisterC(0),
                program: Program(vec![
                    (Instruction::Adv, Operand(1)),
                    (Instruction::Out, Operand(4)),
                    (Instruction::Jnz, Operand(0))
                ])
            },
            computer
        );
    }
}
