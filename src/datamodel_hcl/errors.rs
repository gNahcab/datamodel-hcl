use std::io;

#[derive(Debug)]
pub enum DatamodelHCLError {
    IO(io::Error),
    ParseHcl(hcl::Error),
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
