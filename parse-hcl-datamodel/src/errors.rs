use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum DatamodelHCLError {
    IO(io::Error),
    ParseHcl(hcl::Error),
    ParseInt(ParseIntError),
    ParseProjectModel(String),
    ValidationError(String),
}

impl From<io::Error> for DatamodelHCLError {
    fn from(error: io::Error) -> Self {
        DatamodelHCLError::IO(error)
    }
}

impl From<hcl::Error> for DatamodelHCLError {
    fn from(error: hcl::Error) -> Self {
        DatamodelHCLError::ParseHcl(error)
    }
}

impl From<ParseIntError> for DatamodelHCLError {
    fn from(error: ParseIntError) -> Self {
       DatamodelHCLError::ParseInt(error)
    }
}

