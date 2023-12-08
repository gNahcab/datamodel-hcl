use std::io;
use std::num::ParseIntError;
use import_data::errors::DataImportError;

#[derive(Debug)]
pub enum ParseError {
    DataImportError(DataImportError),
    ParseInt(ParseIntError),
    ParseProjectModel(String),
    ValidationError(String),
}


impl From<DataImportError> for ParseError {
    fn from(error: DataImportError) -> Self {
        ParseError::DataImportError(error)
    }
}
impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
       ParseError::ParseInt(error)
    }
}

