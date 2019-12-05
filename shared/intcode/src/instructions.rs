use crate::errors::ProgramError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Argument {
    Position(usize),
    Immediate(i32),
}

impl Argument {
    pub fn get_value(&self, program: &[i32]) -> i32 {
        match *self {
            Argument::Position(index) => program[index],
            Argument::Immediate(value) => value,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Add(Argument, Argument, usize),
    Multiply(Argument, Argument, usize),
    Input(usize),
    Output(Argument),
    Halt,
}

impl Instruction {
    pub fn read(program: &[i32], instruction_ptr: usize) -> Result<Instruction, ProgramError> {
        let instruction = program[instruction_ptr];
        let opcode = instruction % 100;
        let hundreds = (instruction % 1000) - opcode;
        let thousands = (instruction % 10_000) - hundreds - opcode;
        let first_immediate = hundreds > 0;
        let second_immediate = thousands > 0;

        match opcode {
            1 => {
                let a = program[instruction_ptr + 1];
                let a = if first_immediate {
                    Argument::Immediate(a)
                } else {
                    Argument::Position(a as usize)
                };

                let b = program[instruction_ptr + 2];
                let b = if second_immediate {
                    Argument::Immediate(b)
                } else {
                    Argument::Position(b as usize)
                };

                let output_index = program[instruction_ptr + 3] as usize;
                Ok(Instruction::Add(a, b, output_index))
            }
            2 => {
                let a = program[instruction_ptr + 1];
                let a = if first_immediate {
                    Argument::Immediate(a)
                } else {
                    Argument::Position(a as usize)
                };

                let b = program[instruction_ptr + 2];
                let b = if second_immediate {
                    Argument::Immediate(b)
                } else {
                    Argument::Position(b as usize)
                };

                let output_index = program[instruction_ptr + 3] as usize;
                Ok(Instruction::Multiply(a, b, output_index))
            }
            3 => Ok(Instruction::Input(program[instruction_ptr + 1] as usize)),
            4 => {
                let a = program[instruction_ptr + 1];
                let a = if first_immediate {
                    Argument::Immediate(a)
                } else {
                    Argument::Position(a as usize)
                };

                Ok(Instruction::Output(a))
            }
            99 => Ok(Instruction::Halt),
            unknown => Err(ProgramError::UnknownOpcode(unknown)),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) | Instruction::Multiply(_, _, _) => 4,
            Instruction::Input(_) | Instruction::Output(_) => 2,
            Instruction::Halt => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_instruction_test() {
        let i = Instruction::read(&[1002, 1, 2, 3], 0).unwrap();
        assert_eq!(
            i,
            Instruction::Multiply(Argument::Position(1), Argument::Immediate(2), 3)
        );
    }
}
