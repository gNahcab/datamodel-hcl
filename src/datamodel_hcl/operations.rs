use crate::load_datamodel; 
use crate::errors::DatamodelHCLError;
use std::path::Path;


pub fn import<P: AsRef<Path>>(project_path: &P) -> Result<(), DatamodelHCLError> {
    println!("hello from import");
    load_datamodel(project_path)?;
    
    Ok(())
}
