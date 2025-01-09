use std::{num::ParseIntError, str::FromStr};

pub const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

fn main() {
    let puzzle_input = "Register A: 62769524
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,0,3,4,0,1,7,5,5,3,0";

    println!("Day 17");

    println!(
        "Part 1: {}",
        puzzle_input.parse::<Computer>().unwrap().run_program()
    )
}

#[derive(Debug, Clone, PartialEq)]
pub struct Computer {
    pub a: RegisterA,
    pub b: RegisterB,
    pub c: RegisterC,
    pub program: Program,
}

impl Computer {
    pub fn operand_value(&self, operand: Operand) -> u32 {
        match operand.0 {
            n if (0..=3).contains(&n) => n as u32,
            4 => self.a.0,
            5 => self.b.0,
            6 => self.c.0,
            7 => 7,
            _ => panic!("Never happens"),
        }
    }

    pub fn run_program(&mut self) -> String {
        let mut output: Vec<u32> = Vec::new();
        let mut instruction_pointer = 0;

        let program = self.program.clone();

        while let Some(instruction_pair) = program.0.get(instruction_pointer / 2) {
            if let Some(out) = self.execute_instruction(&mut instruction_pointer, instruction_pair)
            {
                output.push(out);
            }
        }

        output
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn execute_instruction(
        &mut self,
        instruction_pointer: &mut usize,
        (instruction, operand): &(Instruction, Operand),
    ) -> Option<u32> {
        let value = self.operand_value(*operand);
        *instruction_pointer += 2;
        match instruction {
            Instruction::Adv => self.a.0 /= 2u32.pow(value),
            Instruction::Bxl => self.b.0 ^= value,
            Instruction::Bst => self.b.0 = value % 8,
            Instruction::Jnz => {
                if self.a.0 == 0 {
                    return None;
                }

                *instruction_pointer = value as usize;
            }
            Instruction::Bxc => self.b.0 ^= self.c.0,
            Instruction::Out => return Some(value % 8),
            Instruction::Bdv => self.b.0 = self.a.0 / 2u32.pow(value),
            Instruction::Cdv => self.c.0 = self.a.0 / 2u32.pow(value),
        }

        None
    }
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
        let (registers_input, program_input) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| ParseComputerError::InvalidInput(s.to_owned()))?;

        let mut a = RegisterA(0);
        let mut b = RegisterB(0);
        let mut c = RegisterC(0);

        for line in registers_input.lines() {
            if line.contains('A') {
                a = line
                    .parse()
                    .map_err(|err| ParseComputerError::ParseRegister('A', err))?;
            }

            if line.contains('B') {
                b = line
                    .parse()
                    .map_err(|err| ParseComputerError::ParseRegister('B', err))?;
            }

            if line.contains('C') {
                c = line
                    .parse()
                    .map_err(|err| ParseComputerError::ParseRegister('C', err))?;
            }
        }

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

#[derive(Debug, Clone, Copy, PartialEq)]
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

    #[test]
    fn execute() {
        let mut computer: Computer = "Register C: 9\n\nProgram: 2,6".parse().unwrap();
        computer.run_program();
        assert_eq!(RegisterB(1), computer.b);

        let mut computer: Computer = "Register A: 10\n\nProgram: 5,0,5,1,5,4".parse().unwrap();
        assert_eq!("0,1,2".to_owned(), computer.run_program());

        let mut computer: Computer = "Register A: 2024\n\nProgram: 0,1,5,4,3,0".parse().unwrap();
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0".to_owned(), computer.run_program());

        let mut computer: Computer = "Register B: 29\n\nProgram: 1,7".parse().unwrap();
        computer.run_program();
        assert_eq!(RegisterB(26), computer.b);

        let mut computer: Computer = "Register B: 2024\nRegister C: 43690\n\nProgram: 4,0"
            .parse()
            .unwrap();
        computer.run_program();
        assert_eq!(RegisterB(44354), computer.b);

        let mut computer: Computer = EXAMPLE.parse().unwrap();
        assert_eq!("4,6,3,5,6,3,5,2,1,0", computer.run_program())
    }
}
