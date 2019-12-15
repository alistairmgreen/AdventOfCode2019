use intcode::{IntcodeMachine, ProgramError, ProgramState};
use std::collections::HashMap;

fn main() -> Result<(), ProgramError> {
    let program: Vec<i64> = include_str!("puzzle_input.txt")
    .split(",")
    .map(|n| n.parse().unwrap())
    .collect();

    let mut machine = IntcodeMachine::new(program);
    let mut picture = Picture::new();

    loop {
        match machine.run()? {
            ProgramState::Completed(outputs) => {
                picture.paint(&outputs);
                break;
            },
            ProgramState::PendingInput(outputs) => {
                picture.paint(&outputs);

                machine.add_input(match picture.current_colour() {
                    Colour::Black => 0,
                    Colour::White => 1,
                });
            }
        }
    }

    println!("{} squares are painted at least once.", picture.pixels.len());

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Colour {
    Black,
    White,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

struct Picture {
    pixels: HashMap<(i32, i32), Colour>,
    x: i32,
    y: i32,
    direction: Direction,
}

impl Picture {
    pub fn new() -> Picture {
        Picture {
            pixels: HashMap::new(),
            x: 0,
            y: 0,
            direction: Direction::Up
        }
    }

    pub fn current_colour(&self) -> Colour {
        *self.pixels.get(&(self.x, self.y))
            .unwrap_or(&Colour::Black)
    }

    pub fn paint(&mut self, instructions: &[i64]) {
        for instruction in instructions.chunks(2) {
            let colour = instruction[0];
            let turn = instruction[1];
            self.pixels.insert((self.x, self.y), match colour {
                0 => Colour::Black,
                1 => Colour::White,
                other => panic!("Unexpected colour value {}", other)
            });

            let next_direction = match turn {
                0 => self.direction.left(),
                1 => self.direction.right(),
                other => panic!("Unexpected turn direction {}", other)
            };

            self.direction = next_direction;

            let (x, y) = match self.direction {
                Direction::Up => (self.x, self.y - 1),
                Direction::Right => (self.x + 1, self.y),
                Direction::Down => (self.x, self.y + 1),
                Direction::Left => (self.x - 1, self.y)
            };

            self.x = x;
            self.y = y;
        }
    }
}