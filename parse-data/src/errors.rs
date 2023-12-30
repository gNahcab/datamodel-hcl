use std::io;
use std::num::ParseIntError;
use import_data::errors::DataImportError;

#[derive(Debug)]
pub enum ParsingError {
    DataImportError(DataImportError),
    ParseInt(ParseIntError),
    ParseProjectModel(String),
    ValidationError(String),
    XlsxParse(String),
    RegexError(regex::Error),
}


impl From<DataImportError> for ParsingError {
    fn from(error: DataImportError) -> Self {
        ParsingError::DataImportError(error)
    }
}
impl From<ParseIntError> for ParsingError {
    fn from(error: ParseIntError) -> Self {
       ParsingError::ParseInt(error)
    }
}

impl From<regex::Error> for ParsingError {
    fn from(error: regex::Error) -> Self {
        ParsingError::RegexError(error)
    }
}
