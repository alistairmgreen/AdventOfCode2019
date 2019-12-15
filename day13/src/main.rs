use crossterm::event::{read, Event};
use crossterm::{cursor, execute, queue, style, terminal};
use intcode::{IntcodeMachine, ProgramState};
use std::cmp::Ordering;
use std::{fmt, thread, time::Duration};
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine: IntcodeMachine = include_str!("puzzle_input.txt").parse()?;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )?;

    'game: loop {
        match machine.run()? {
            ProgramState::Completed(output) => {
                let tiles = output.chunks(3);
                for tile in tiles {
                    if tile[0] < 0 {
                        score = tile[2];

                        execute!(
                            stdout,
                            cursor::MoveTo(43, 0),
                            style::Print(format!("Score: {:>6}", score))
                        )?;

                        continue;
                    }

                    let position = (tile[0] as u16, tile[1] as u16);
                    let tile_type = match tile[2] {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        other => panic!("Unexpected tile type {}", other),
                    };

                    queue!(
                        stdout,
                        cursor::MoveTo(position.0, position.1),
                        style::Print(tile_type.to_string())
                    )?;
                }
                stdout.flush()?;

                break 'game;
            }
            ProgramState::PendingInput(output) => {
                let tiles = output.chunks(3);
                for tile in tiles {
                    if tile[0] < 0 {
                        score = tile[2];

                        execute!(
                            stdout,
                            cursor::MoveTo(43, 0),
                            style::Print(format!("Score: {:>6}", score))
                        )?;

                        continue;
                    }

                    let position = (tile[0] as u16, tile[1] as u16);

                    let tile_type = match tile[2] {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        other => panic!("Unexpected tile type {}", other),
                    };

                    if tile_type == Tile::Ball {
                        ball_x = position.0;
                    }

                    if tile_type == Tile::Paddle {
                        paddle_x = position.0;
                    }

                    execute!(
                        stdout,
                        cursor::MoveTo(position.0, position.1),
                        style::Print(tile_type.to_string())
                    )?;
                }

                thread::sleep(Duration::from_millis(10));

                match paddle_x.cmp(&ball_x) {
                    Ordering::Less => {
                        machine.add_input(1);
                    }
                    Ordering::Equal => {
                        machine.add_input(0);
                    }
                    Ordering::Greater => {
                        machine.add_input(-1);
                    }
                }
            }
        }
    }

    loop {
        if let Event::Key(_) = read()? {
            break;
        }
    }

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::Show
    )?;

    println!("Final score: {}", score);

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
            Tile::Ball => write!(f, "O"),
        }
    }
}
