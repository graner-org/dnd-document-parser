use serde_json::Error as JSONError;
use std::io::Error as IOError;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub string: String,
    pub parsing_step: String,
    pub problem: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct OutOfBoundsError {
    pub array: Vec<String>,
    pub parsing_step: String,
    pub problem: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    JSON(JSONError),
    OutOfBounds(OutOfBoundsError),
    Parse(ParseError),
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Error::IO(value)
    }
}

impl From<JSONError> for Error {
    fn from(value: JSONError) -> Self {
        Error::JSON(value)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::Parse(value)
    }
}

impl From<OutOfBoundsError> for Error {
    fn from(value: OutOfBoundsError) -> Self {
        Error::OutOfBounds(value)
    }
}
