use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    IO(io::Error),
    ParseHcl(hcl::Error),
    ParseInt(ParseIntError),
    ParseProjectModel(String),
    ValidationError(String),
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        ParseError::IO(error)
    }
}

impl From<hcl::Error> for ParseError {
    fn from(error: hcl::Error) -> Self {
        ParseError::ParseHcl(error)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
       ParseError::ParseInt(error)
    }
}

