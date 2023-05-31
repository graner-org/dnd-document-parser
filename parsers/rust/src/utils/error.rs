#![allow(clippy::module_name_repetitions)]
use serde_json::{Error as JSONError, Value};
use std::{io::Error as IOError, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub struct JSONDiffError {
    pub json1: Value,
    pub json2: Value,
    pub json_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub string: String,
    pub parsing_step: String,
    pub problem: Option<String>,
}

impl ParseError {
    pub fn from_intparse_error(
        string: String,
        parsing_step: String,
    ) -> impl FnOnce(ParseIntError) -> Self {
        |error: ParseIntError| Self {
            string,
            parsing_step,
            problem: Some(error.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
        use Error::{OutOfBounds, Parse};
        match (self, other) {
            (Parse(self_parse), Parse(other_parse)) => self_parse == other_parse,
            (OutOfBounds(self_oob), OutOfBounds(other_oob)) => self_oob == other_oob,
            _ => false,
        }
    }
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IO(value)
    }
}

impl From<JSONError> for Error {
    fn from(value: JSONError) -> Self {
        Self::JSON(value)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<OutOfBoundsError> for Error {
    fn from(value: OutOfBoundsError) -> Self {
        Self::OutOfBounds(value)
    }
}
