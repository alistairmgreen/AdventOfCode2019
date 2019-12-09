use crate::errors::ProgramError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Argument {
    Position(usize),
    Immediate(i64),
}

impl Argument {
    pub fn get_value(&self, program: &[i64]) -> i64 {
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
    JumpIfTrue(Argument, Argument),
    JumpIfFalse(Argument, Argument),
    LessThan(Argument, Argument, usize),
    Equals(Argument, Argument, usize),
    Halt,
}

impl Instruction {
    pub fn read(program: &[i64], instruction_ptr: usize) -> Result<Instruction, ProgramError> {
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
            5 => {
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

                Ok(Instruction::JumpIfTrue(a, b))
            }
            6 => {
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

                Ok(Instruction::JumpIfFalse(a, b))
            }
            7 => {
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
                Ok(Instruction::LessThan(a, b, output_index))
            }
            8 => {
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
                Ok(Instruction::Equals(a, b, output_index))
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
            Instruction::Input(_) | Instruction::Output(_) => 2,
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
        let i = Instruction::read(&[1002, 1, 2, 3], 0).unwrap();
        assert_eq!(
            i,
            Instruction::Multiply(Argument::Position(1), Argument::Immediate(2), 3)
        );
    }
}
