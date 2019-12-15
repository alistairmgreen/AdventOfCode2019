use intcode::{IntcodeMachine, ProgramState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine: IntcodeMachine = include_str!("puzzle_input.txt").parse()?;

    match machine.run()? {
        ProgramState::Completed(output) => {
            let tiles = output.chunks(3);
            let mut blocks = 0;
            for tile in tiles {
                if tile[2] == 2 {
                    blocks += 1;
                }
            }

            println!("There are {} block tiles.", blocks);
        }
        ProgramState::PendingInput(_) => {
            eprintln!("Program is waiting for input.");
        }
    }

    Ok(())
}
