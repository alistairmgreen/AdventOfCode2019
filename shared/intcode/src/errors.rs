use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramError {
    UnknownOpcode(i64),
    IndexOutOfRange(usize),
    InsufficientInput,
    UnknownParameterMode(i64),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::UnknownOpcode(code) => write!(f, "Unknown opcode: {}", code),
            ProgramError::IndexOutOfRange(index) => write!(f, "Index out of range: {}", index),
            ProgramError::InsufficientInput => write!(f, "Not enough input values supplied"),
            ProgramError::UnknownParameterMode(mode) => write!(f, "Unknown parameter mode {}", mode),
        }
    }
}

impl Error for ProgramError {}