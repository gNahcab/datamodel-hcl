use std::path::Path;
use crate::errors::DataImportError;

pub fn read_hcl_body<P: AsRef<Path>>(path: P) -> Result<hcl::Body, DataImportError> {
    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(error) => return Err(DataImportError::IO(error)) ,
    };
    let body:hcl::Body = hcl::from_str(&inputstr)?;
    Ok(body)
}