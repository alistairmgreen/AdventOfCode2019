use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramError {
    UnknownOpcode(i32),
    IndexOutOfRange(usize),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::UnknownOpcode(code) => write!(f, "Program alarm: {}", code),
            ProgramError::IndexOutOfRange(index) => write!(f, "Index out of range: {}", index),
        }
    }
}

impl Error for ProgramError {}