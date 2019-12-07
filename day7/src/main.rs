use day7::permutation::Permutations;
use intcode::*;

fn main() {
    let program = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 63, 76, 101, 114, 195, 276, 357, 438, 99999, 3,
        9, 101, 2, 9, 9, 102, 5, 9, 9, 1001, 9, 3, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9,
        102, 5, 9, 9, 1001, 9, 5, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 3, 9, 1002, 9, 5, 9, 4,
        9, 99, 3, 9, 1002, 9, 2, 9, 101, 5, 9, 9, 102, 3, 9, 9, 101, 2, 9, 9, 1002, 9, 3, 9, 4, 9,
        99, 3, 9, 101, 3, 9, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
        9, 3, 9, 101, 2, 9, 9, 4, 9, 99,
    ];

    println!("Part 1:");

    let (phases, signal) = Permutations::of(vec![0, 1, 2, 3, 4])
        .map(|phases| {
            let signal = thruster_signal(&program, &phases).unwrap();
            (phases, signal)
        })
        .max_by_key(|(_, signal)| *signal)
        .unwrap();

    println!("Phases: {:?} give signal strength {}", phases, signal);

    println!("Part 2:");

    let (phases, signal) = Permutations::of(vec![5, 6, 7, 8, 9])
        .map(|phases| {
            let signal = feedback_loop(&program, &phases).unwrap();
            (phases, signal)
        })
        .max_by_key(|(_, signal)| *signal)
        .unwrap();

    println!("Phases: {:?} give signal strength {}", phases, signal);
}

fn thruster_signal(program: &[i32], phases: &[i32]) -> Result<i32, ProgramError> {
    let mut input = 0;
    for phase in phases {
        let output = run(&mut program.to_owned(), vec![*phase, input])?;
        input = output[0];
    }

    Ok(input)
}

fn feedback_loop(program: &[i32], phases: &[i32]) -> Result<i32, ProgramError> {
    let mut amplifiers: Vec<IntcodeMachine> = phases
        .iter()
        .map(|&phase| IntcodeMachine::with_seed(program.to_owned(), phase))
        .collect();
    let last_amplifier = amplifiers.len() - 1;
    let mut previous_output = vec![0];

    loop {
        for (index, amplifier) in amplifiers.iter_mut().enumerate() {
            amplifier.add_inputs(previous_output);
            let output = amplifier.run()?;

            match output {
                ProgramState::Completed(mut values) if index == last_amplifier => {
                    return Ok(values.pop().expect("No output from last amplifier!"));
                }
                ProgramState::PendingInput(values) | ProgramState::Completed(values) => {
                    previous_output = values;
                }
            }
        }
    }
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

    #[test]
    fn part2_example1() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let result = feedback_loop(&program, &vec![9, 8, 7, 6, 5]);
        assert_eq!(result, Ok(139629729))
    }

    #[test]
    fn part2_example2() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let result = feedback_loop(&program, &vec![9, 7, 8, 5, 6]);
        assert_eq!(result, Ok(18216))
    }
}
