use intcode::{IntcodeMachine, ProgramState};
use std::fmt;
use std::collections::HashMap;
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, execute, queue, terminal, style};
use crossterm::event::{read, Event};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine: IntcodeMachine = include_str!("puzzle_input.txt").parse()?;
    let mut screen: HashMap<(u16, u16), Tile> = HashMap::new();

    match machine.run()? {
        ProgramState::Completed(output) => {
            let tiles = output.chunks(3);
            for tile in tiles {
                let position = (tile[0] as u16, tile[1] as u16);
                let tile_type = match tile[2] {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::Paddle,
                    4 => Tile::Ball,
                    other => panic!("Unexpected tile type {}", other)
                };

                screen.insert(position, tile_type);
            }
        }
        ProgramState::PendingInput(_) => {
            eprintln!("Program is waiting for input.");
        }
    }

    let mut stdout = stdout();
    queue!(stdout, terminal::Clear(terminal::ClearType::All), cursor::Hide)?;

    for (&(x, y), &tile_type) in screen.iter() {
        queue!(
            stdout,
            cursor::MoveTo(x, y),
            style::Print(tile_type.to_string())
        )?;
    }

    let block_count = screen.values()
        .filter(|&&tile| tile == Tile::Block)
        .count();
    

    stdout.flush()?;

    loop {
        if let Event::Key(_) = read()? {
            break;
        }
    }

    execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::Show)?;

    println!("There are {} block tiles.", block_count);

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "X"),
            Tile::Block => write!(f, "*"),
            Tile::Paddle => write!(f, "_"),
            Tile::Ball => write!(f, "O")
        }
    }
}