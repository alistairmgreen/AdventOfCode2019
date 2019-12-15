use intcode::{IntcodeMachine, ProgramError, ProgramState};
use std::collections::HashMap;
use std::fmt;

fn main() -> Result<(), ProgramError> {
    let program: Vec<i64> = include_str!("puzzle_input.txt")
    .split(",")
    .map(|n| n.parse().unwrap())
    .collect();

    let part1 = paint_picture(program.clone(), Colour::Black)?;
    println!("Part 1: {} squares are painted at least once.", part1.pixels.len());

    println!("Part 2:");
    let part2 = paint_picture(program, Colour::White)?;
    println!("{}", part2);

    Ok(())
}

fn paint_picture(program: Vec<i64>, initial_colour: Colour) -> Result<Picture, ProgramError> {
    let mut machine = IntcodeMachine::new(program);
    let mut picture = Picture::new();

    if initial_colour == Colour::White {
        picture.pixels.insert((0, 0), Colour::White);
    }

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

    Ok(picture)
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

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for &(x, y) in self.pixels.keys() {
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pixel = match self.pixels.get(&(x, y)) {
                    Some(Colour::White) => '#',
                    _ => ' '
                };
                write!(f, "{}", pixel)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}