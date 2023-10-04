use crate::{load_datamodel, validate};
use crate::errors::DatamodelHCLError;
use std::path::Path;
use crate::domain::project_model::ProjectModel;


pub fn import<P: AsRef<Path>>(project_path: &P) -> () {
    let project: Result<ProjectModel, DatamodelHCLError> =  load_datamodel(project_path);
    validate(project.unwrap());
}



#[cfg(test)]
mod test {
    use crate::load_datamodel;

    #[test]
    fn test_import() {
        let path =  "data/testdata/rosetta.hcl";
        let result = load_datamodel(path);
        assert!(result.is_ok())
    }



}
