use std::fmt;
use std::error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum StepParseError {
    InvalidDirection,
    InvalidNumber,
}

impl fmt::Display for StepParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StepParseError::InvalidDirection => write!(f, "Invalid direction"),
            StepParseError::InvalidNumber => write!(f, "Invalid number")
        }
    }
}

impl error::Error for StepParseError {}

impl From<ParseIntError> for StepParseError {
    fn from(_error: ParseIntError) -> Self {
        StepParseError::InvalidNumber
    }
}