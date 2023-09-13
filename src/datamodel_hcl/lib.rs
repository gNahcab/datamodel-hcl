pub mod errors;
pub mod domain;
use crate::errors::DatamodelHCLError;
pub mod operations;
use std::path::Path;
use clap::builder::Str;
use hcl::Error;
use hcl::Value::String;
use crate::domain::project_model::ProjectModel;


pub fn load_datamodel<P: AsRef<Path>>(path: P) -> () {
    let input = std::fs::read_to_string(path);
    let inputstr = match input {
        Ok(str_) => str_,
        Err(_) => std::string::String::from("found error"),
    };
    let body:hcl::Body = hcl::from_str(&inputstr).expect("couldn't parse body");
    // call parser method
    let result: ProjectModel= body.try_into().unwrap();
}




