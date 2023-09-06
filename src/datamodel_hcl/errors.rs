use std::io;
use clap::builder::Str;

#[derive(Debug)]
pub enum DatamodelHCLError {
    IO(io::Error),
    ParseHcl(hcl::Error),
    ParseProjectInfo(String),
    ParsePassword(String),
    ParseShortname(String),
    ParseShortcode(String),
    ParseLongname(String),
    ParseKeywords(String),
    ParseDescriptions(String),
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

