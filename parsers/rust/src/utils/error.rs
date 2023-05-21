use serde_json::{Error as JSONError, Value};
use std::{io::Error as IOError, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub struct JSONDiffError {
    pub json1: Value,
    pub json2: Value,
    pub json_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    pub string: String,
    pub parsing_step: String,
    pub problem: Option<String>,
}

impl ParseError {
    pub fn from_intparse_error(
        string: String,
        parsing_step: String,
    ) -> impl FnOnce(ParseIntError) -> ParseError {
        |error: ParseIntError| ParseError {
            string,
            parsing_step,
            problem: Some(error.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OutOfBoundsError {
    pub array: Vec<String>,
    pub index: u32,
    pub parsing_step: String,
}

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    JSON(JSONError),
    OutOfBounds(OutOfBoundsError),
    Parse(ParseError),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;
        match (self, other) {
            (Parse(self_parse), Parse(other_parse)) => self_parse == other_parse,
            (OutOfBounds(self_oob), OutOfBounds(other_oob)) => self_oob == other_oob,
            _ => false,
        }
    }
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
