use std::path::Path;
use crate::errors::DataImportError;
use crate::errors::DataImportError::IO;

pub fn read_hcl_body<P: AsRef<Path>>(path: P) -> Result<hcl::Body, Err> {
    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(error) => return Err(IO::from(error)) ,
    };
    let body:hcl::Body = hcl::from_str(&inputstr)?;
    Ok(body)
}