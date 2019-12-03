pub mod errors;
pub use crate::errors::StepParseError;
use core::str::FromStr;
use std::ops::{Add, AddAssign};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;
    fn add(self, other: (i32, i32)) -> Self {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, other: (i32, i32)) {
        self.x += other.0;
        self.y += other.1;
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    pub fn displacement(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = StepParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('U') => Ok(Direction::Up),
            Some('R') => Ok(Direction::Right),
            Some('L') => Ok(Direction::Left),
            Some('D') => Ok(Direction::Down),
            _ => Err(StepParseError::InvalidDirection),
        }
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WireSegment {
    pub direction: Direction,
    pub distance: i32,
}

impl FromStr for WireSegment {
    type Err = StepParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: Direction = s.parse()?;
        let distance: i32 = s.chars().skip(1).collect::<String>().parse()?;
        
        Ok(WireSegment { direction, distance })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wire_segment() {
        let s: WireSegment = "R995".parse().unwrap();
        assert_eq!(s, WireSegment { direction: Direction::Right, distance: 995 });
    }
}