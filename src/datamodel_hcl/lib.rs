pub mod errors;
pub mod domain;
use crate::errors::DatamodelHCLError;
pub mod operations;
use std::path::Path;




pub fn load_datamodel<P: AsRef<Path>>(path: P) -> Result<(), DatamodelHCLError> {
    let input = std::fs::read_to_string(path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    // call parser method
    Ok(())
}

#[cfg(test)]
mod tests {

    }


