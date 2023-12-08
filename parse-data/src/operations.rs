use crate::datamodel_parse::domain::project_model::ProjectModel;
use crate::errors::ParseError;
use std::path::Path;
use import_data::operations::load_hcl;
pub fn load_datamodel<P: AsRef<Path>>(path: P) -> Result<ProjectModel, ParseError> {
    // takes a path to hcl-file
    // if the hcl-file was correctly formed it returns a ProjectModel, otherwise a DatamodelHCLError
    //let body = import_data::importers::
    //let body = hcl_import::read_hcl::read_hcl(path);
    // call parser method
    let body = load_hcl(path);
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
