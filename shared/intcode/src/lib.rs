pub mod errors;
pub mod instructions;
pub use crate::errors::ProgramError;
use instructions::Instruction;

pub fn run<T>(program: &mut [i32], input: T) -> Result<Vec<i32>, ProgramError>
where
    T: IntoIterator<Item = i32>,
{
    let mut inputs = input.into_iter();
    let mut outputs = Vec::new();
    let mut instruction_ptr = 0;
    let program_length = program.len();
    let mut jumped: bool;

    while instruction_ptr < program_length {
        jumped = false;
        let instruction = Instruction::read(program, instruction_ptr)?;

        match instruction {
            Instruction::Add(a, b, out) => {
                program[out] = a.get_value(program) + b.get_value(program);
            }
            Instruction::Multiply(a, b, out) => {
                program[out] = a.get_value(program) * b.get_value(program);
            }
            Instruction::Input(destination) => {
                program[destination] = inputs.next().ok_or(ProgramError::InsufficientInput)?;
            }
            Instruction::Output(value) => {
                outputs.push(value.get_value(program));
            }
            Instruction::JumpIfFalse(value, destination) => {
                if value.get_value(program) == 0 {
                    instruction_ptr = destination.get_value(program) as usize;
                    jumped = true;
                }
            }
            Instruction::JumpIfTrue(value, destination) => {
                if value.get_value(program) != 0 {
                    instruction_ptr = destination.get_value(program) as usize;
                    jumped = true;
                }
            }
            Instruction::LessThan(a, b, destination) => {
                program[destination] = if a.get_value(program) < b.get_value(program) {
                    1
                } else {
                    0
                };
            }
            Instruction::Equals(a, b, destination) => {
                program[destination] = if a.get_value(program) == b.get_value(program) {
                    1
                } else {
                    0
                };
            }
            Instruction::Halt => {
                break;
            }
        }

        if !jumped {
            instruction_ptr += instruction.arity();
        }
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn example1() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = run(&mut program, iter::empty());
        assert!(result.is_ok());
        assert_eq!(program[0], 3500);
    }

    #[test]
    fn example2() {
        let mut program = vec![1, 0, 0, 0, 99];
        let result = run(&mut program, iter::empty());
        assert!(result.is_ok());
        assert_eq!(program[0], 2);
    }

    #[test]
    fn example3() {
        let mut program = vec![2, 3, 0, 3, 99];
        let result = run(&mut program, iter::empty());
        assert!(result.is_ok());
        assert_eq!(program[0], 2)
    }

    #[test]
    fn example4() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        let result = run(&mut program, iter::empty());
        assert!(result.is_ok());
        assert_eq!(program[0], 2)
    }

    #[test]
    fn example5() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = run(&mut program, iter::empty());
        assert!(result.is_ok());
        assert_eq!(program[0], 30)
    }

    #[test]
    fn day2_solution() {
        let mut program = vec![
            1, 31, 46, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 6, 19, 23, 2, 23, 6,
            27, 1, 5, 27, 31, 1, 31, 9, 35, 2, 10, 35, 39, 1, 5, 39, 43, 2, 43, 10, 47, 1, 47, 6,
            51, 2, 51, 6, 55, 2, 55, 13, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 6, 67, 71, 2, 71, 9,
            75, 1, 6, 75, 79, 2, 13, 79, 83, 1, 9, 83, 87, 1, 87, 13, 91, 2, 91, 10, 95, 1, 6, 95,
            99, 1, 99, 13, 103, 1, 13, 103, 107, 2, 107, 10, 111, 1, 9, 111, 115, 1, 115, 10, 119,
            1, 5, 119, 123, 1, 6, 123, 127, 1, 10, 127, 131, 1, 2, 131, 135, 1, 135, 10, 0, 99, 2,
            14, 0, 0,
        ];

        let result = run(&mut program, iter::empty());
        assert!(result.is_ok());
        assert_eq!(program[0], 1969_07_20);
    }

    #[test]
    fn day5_solution() {
        let mut program = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1, 191, 196, 224, 1001, 224, -85, 224,
            4, 224, 1002, 223, 8, 223, 1001, 224, 4, 224, 1, 223, 224, 223, 1101, 45, 50, 225,
            1102, 61, 82, 225, 101, 44, 39, 224, 101, -105, 224, 224, 4, 224, 102, 8, 223, 223,
            101, 5, 224, 224, 1, 224, 223, 223, 102, 14, 187, 224, 101, -784, 224, 224, 4, 224,
            102, 8, 223, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1001, 184, 31, 224, 1001, 224,
            -118, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1102, 91, 18,
            225, 2, 35, 110, 224, 101, -810, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224,
            1, 223, 224, 223, 1101, 76, 71, 224, 1001, 224, -147, 224, 4, 224, 102, 8, 223, 223,
            101, 2, 224, 224, 1, 224, 223, 223, 1101, 7, 16, 225, 1102, 71, 76, 224, 101, -5396,
            224, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223, 223, 1101, 72, 87,
            225, 1101, 56, 77, 225, 1102, 70, 31, 225, 1102, 29, 15, 225, 1002, 158, 14, 224, 1001,
            224, -224, 224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 223, 224, 223, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999,
            1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 329, 1001, 223, 1, 223, 8, 226, 677,
            224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 107, 226, 677, 224, 1002,
            223, 2, 223, 1006, 224, 359, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2, 223,
            1005, 224, 374, 1001, 223, 1, 223, 1108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224,
            389, 1001, 223, 1, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 404, 101, 1,
            223, 223, 7, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 419, 1001, 223, 1, 223, 1108,
            226, 677, 224, 102, 2, 223, 223, 1005, 224, 434, 1001, 223, 1, 223, 1107, 226, 226,
            224, 1002, 223, 2, 223, 1006, 224, 449, 1001, 223, 1, 223, 1007, 677, 677, 224, 102, 2,
            223, 223, 1006, 224, 464, 1001, 223, 1, 223, 107, 226, 226, 224, 1002, 223, 2, 223,
            1005, 224, 479, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224,
            494, 1001, 223, 1, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 509, 101, 1,
            223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 524, 1001, 223, 1, 223,
            1108, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 539, 1001, 223, 1, 223, 7, 226, 677,
            224, 102, 2, 223, 223, 1006, 224, 554, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223,
            2, 223, 1006, 224, 569, 101, 1, 223, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006,
            224, 584, 1001, 223, 1, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 599,
            101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 614, 1001, 223, 1,
            223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 1001, 223, 1, 223, 108,
            677, 226, 224, 102, 2, 223, 223, 1005, 224, 644, 101, 1, 223, 223, 1008, 226, 677, 224,
            1002, 223, 2, 223, 1005, 224, 659, 101, 1, 223, 223, 108, 677, 677, 224, 1002, 223, 2,
            223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
        ];

        let result = run(&mut program, iter::once(5))
            .unwrap();
        
        assert_eq!(result, vec![4283952]);
    }
}
