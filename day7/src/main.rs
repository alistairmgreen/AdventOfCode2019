use intcode::*;

fn main() {
    println!("Hello, world!");
}

fn thruster_signal(program: &[i32], phases: &[i32]) -> Result<i32, ProgramError> {
    let mut input = 0;
    for phase in phases {
        let output = run(&mut program.to_owned(), vec![*phase, input])?;
        input = output[0];
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases = vec![4, 3, 2, 1, 0];
        assert_eq!(thruster_signal(&program, &phases), Ok(43210));
    }

    #[test]
    fn part1_example2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = vec![0, 1, 2, 3, 4];
        assert_eq!(thruster_signal(&program, &phases), Ok(54321));
    }

    #[test]
    fn part1_example3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = vec![1, 0, 4, 3, 2];
        assert_eq!(thruster_signal(&program, &phases), Ok(65210));
    }
}
