pub mod errors;
pub mod instructions;
pub use crate::errors::ProgramError;
use instructions::Instruction;

pub fn run(program: &mut [i32]) -> Result<(), ProgramError> {
    let mut instruction_ptr = 0;
    let program_length = program.len();
    while instruction_ptr < program_length {
        let instruction = Instruction::read(program, instruction_ptr)?;

        match instruction {
            Instruction::Add(a, b, out) => {
                program[out] = a.get_value(program) + b.get_value(program);
            }
            Instruction::Multiply(a, b, out) => {
                program[out] = a.get_value(program) * b.get_value(program);
            }
            Instruction::Input(destination) => {
                program[destination] = 1;
            }
            Instruction::Output(value) => {
                println!("{}", value.get_value(program));
            }
            Instruction::Halt => {
                break;
            }
        }

        instruction_ptr += instruction.arity();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = run(&mut program);
        assert!(result.is_ok());
        assert_eq!(program[0], 3500);
    }

    #[test]
    fn example2() {
        let mut program = vec![1, 0, 0, 0, 99];
        let result = run(&mut program);
        assert!(result.is_ok());
        assert_eq!(program[0], 2);
    }

    #[test]
    fn example3() {
        let mut program = vec![2, 3, 0, 3, 99];
        let result = run(&mut program);
        assert!(result.is_ok());
        assert_eq!(program[0], 2)
    }

    #[test]
    fn example4() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        let result = run(&mut program);
        assert!(result.is_ok());
        assert_eq!(program[0], 2)
    }

    #[test]
    fn example5() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = run(&mut program);
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

        let result = run(&mut program);
        assert!(result.is_ok());
        assert_eq!(program[0], 1969_07_20);
    }
}
