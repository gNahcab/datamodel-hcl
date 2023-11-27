pub mod errors;
pub mod domain;
use crate::errors::DatamodelHCLError;
pub mod operations;
use std::path::Path;
use crate::domain::project_model::ProjectModel;


pub fn load_datamodel<P: AsRef<Path>>(path: P) -> Result<ProjectModel, DatamodelHCLError> {

    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(_) => std::string::String::from("input string error..is path correct?"),
    };
    let body:hcl::Body = hcl::from_str(&inputstr).expect("couldn't parse body");
    // call parser method
    let datamodel: ProjectModel = body.try_into()?;
    Ok(datamodel)
}






