use crate::errors::ProgramError;
use crate::ProgramStore;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Argument {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl Argument {
    fn new(modes: i64, argument_number: u32, value: i64) -> Result<Argument, ProgramError> {
        let x = 10i64.pow(argument_number + 2);
        let parameter_mode = ((modes % 10i64.pow(argument_number + 3)) - (modes % x)) / x;
        match parameter_mode {
            0 => Ok(Argument::Position(value as usize)),
            1 => Ok(Argument::Immediate(value)),
            2 => Ok(Argument::Relative(value)),
            other => Err(ProgramError::UnknownParameterMode(other)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Add(Argument, Argument, Argument),
    Multiply(Argument, Argument, Argument),
    Input(Argument),
    Output(Argument),
    JumpIfTrue(Argument, Argument),
    JumpIfFalse(Argument, Argument),
    LessThan(Argument, Argument, Argument),
    Equals(Argument, Argument, Argument),
    SetRelativeBase(Argument),
    Halt,
}

impl Instruction {
    pub fn read(
        program: &ProgramStore,
        instruction_ptr: usize,
    ) -> Result<Instruction, ProgramError> {
        let instruction = program[instruction_ptr];
        let opcode = instruction % 100;
        let modes = instruction - opcode;
        match opcode {
            1 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                let b = Argument::new(modes, 1, program[instruction_ptr + 2])?;
                let c = Argument::new(modes, 2, program[instruction_ptr + 3])?;
                Ok(Instruction::Add(a, b, c))
            }
            2 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                let b = Argument::new(modes, 1, program[instruction_ptr + 2])?;
                let c = Argument::new(modes, 2, program[instruction_ptr + 3])?;
                Ok(Instruction::Multiply(a, b, c))
            }
            3 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                Ok(Instruction::Input(a))
            }
            4 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                Ok(Instruction::Output(a))
            }
            5 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                let b = Argument::new(modes, 1, program[instruction_ptr + 2])?;
                Ok(Instruction::JumpIfTrue(a, b))
            }
            6 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                let b = Argument::new(modes, 1, program[instruction_ptr + 2])?;
                Ok(Instruction::JumpIfFalse(a, b))
            }
            7 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                let b = Argument::new(modes, 1, program[instruction_ptr + 2])?;
                let c = Argument::new(modes, 2, program[instruction_ptr + 3])?;
                Ok(Instruction::LessThan(a, b, c))
            }
            8 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                let b = Argument::new(modes, 1, program[instruction_ptr + 2])?;
                let c = Argument::new(modes, 2, program[instruction_ptr + 3])?;
                Ok(Instruction::Equals(a, b, c))
            }
            9 => {
                let a = Argument::new(modes, 0, program[instruction_ptr + 1])?;
                Ok(Instruction::SetRelativeBase(a))
            }
            99 => Ok(Instruction::Halt),
            unknown => Err(ProgramError::UnknownOpcode(unknown)),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Instruction::Add(_, _, _)
            | Instruction::Multiply(_, _, _)
            | Instruction::LessThan(_, _, _)
            | Instruction::Equals(_, _, _) => 4,
            Instruction::Input(_) | Instruction::Output(_) | Instruction::SetRelativeBase(_) => 2,
            Instruction::Halt => 1,
            Instruction::JumpIfFalse(_, _) | Instruction::JumpIfTrue(_, _) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_instruction_test() {
        let program: ProgramStore = vec![1002, 1, 2, 3].into_iter().collect();
        let i = Instruction::read(&program, 0).unwrap();
        assert_eq!(
            i,
            Instruction::Multiply(Argument::Position(1), Argument::Immediate(2), Argument::Position(3))
        );
    }

    #[test]
    fn argument_new_first_parameter() {
        assert_eq!(Argument::new(2000, 0, 1), Ok(Argument::Position(1)));
        assert_eq!(Argument::new(2100, 0, 1), Ok(Argument::Immediate(1)));
        assert_eq!(Argument::new(1200, 0, 1), Ok(Argument::Relative(1)));
    }

    #[test]
    fn argument_new_second_parameter() {
        assert_eq!(Argument::new(10000, 1, 1), Ok(Argument::Position(1)));
        assert_eq!(Argument::new(1000, 1, 1), Ok(Argument::Immediate(1)));
        assert_eq!(Argument::new(2000, 1, 1), Ok(Argument::Relative(1)));
    }
}
