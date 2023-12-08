use crate::datamodel_parse::domain::project_model::ProjectModel;
use crate::errors::ParseError;
use std::path::Path;
use import_data::operations::load_hcl;
use crate::transform_parse::domain::read_transform_hcl::TransformHCL;

pub fn read_datamodel<P: AsRef<Path>>(path: P) -> Result<ProjectModel, ParseError> {
    // if the hcl-file was correctly formed it returns a ProjectModel, otherwise a DatamodelHCLError
    // takes a path to hcl-file
    let body = load_hcl(path)?;
    // call parser method
    let datamodel: ProjectModel = body.try_into()?;
    Ok(datamodel)
}

pub fn read_transform_hcl<P: AsRef<Path>>(path: P) -> Result<TransformHCL, ParseError> {
    let body = load_hcl(path)?;
    let transform_hcl: TransformHCL = body.try_into()?;
    Ok(transform_hcl)
}



#[cfg(test)]
mod test {
    #[test]
    fn test_import() {
        let result =
            super::read_datamodel("../../../data/testdata/rosetta.hcl");
        assert!(result.is_ok());
    }
}