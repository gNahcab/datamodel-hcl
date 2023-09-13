use crate::load_datamodel; 
use crate::errors::DatamodelHCLError;
use std::path::Path;
use crate::domain::project_model::ProjectModel;


pub fn import<P: AsRef<Path>>(project_path: &P) -> () {
    load_datamodel(project_path)
}
