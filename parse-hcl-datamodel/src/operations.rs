use crate::domain::project_model::ProjectModel;
use crate::errors::DatamodelHCLError;
use std::path::Path;
pub fn load_datamodel<P: AsRef<Path>>(path: P) -> Result<ProjectModel, DatamodelHCLError> {
    // takes a path to hcl-file
    // if the hcl-file was correctly formed it returns a ProjectModel, otherwise a DatamodelHCLError

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



#[cfg(test)]
mod test {
    #[test]
    fn test_import() {
        let result =
            super::load_datamodel("../../../data/testdata/rosetta.hcl");
        assert!(result.is_ok());
    }
}
