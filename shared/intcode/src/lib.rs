pub mod errors;
pub use crate::errors::ProgramError;

pub fn run(program: &mut [usize]) -> Result<(), ProgramError> {
    for index in (0..(program.len() - 4)).step_by(4) {
        let opcode = program[index];
        let left_index = program[index + 1];

        let left_operand = *program
            .get(left_index)
            .ok_or(ProgramError::IndexOutOfRange(left_index))?;

        let right_index = program[index + 2];

        let right_operand = *program
            .get(right_index)
            .ok_or(ProgramError::IndexOutOfRange(right_index))?;

        let output_index = program[index + 3];

        match opcode {
            1 => {
                program[output_index] = left_operand + right_operand;
            }
            2 => {
                program[output_index] = left_operand * right_operand;
            }
            99 => {
                // End of program
                return Ok(());
            }
            unknown => {
                return Err(ProgramError::UnknownOpcode(unknown));
            }
        }
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
