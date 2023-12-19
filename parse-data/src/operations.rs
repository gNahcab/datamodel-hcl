use crate::datamodel_parse::domain::project_model::ProjectModel;
use crate::transform_parse::domain::transform_hcl::TransformHCL;
use crate::errors::ParsingError;
use std::path::Path;
use import_data::errors::DataImportError;
use import_data::operations::load_hcl;

pub fn read_datamodel<P: AsRef<Path>>(path: P) -> Result<ProjectModel, ParsingError> {
    // if the hcl-file was correctly formed it returns a ProjectModel, otherwise a DatamodelHCLError
    // takes a path to hcl-file
    let body = load_hcl(path)?;
    // call parser method
    let datamodel: ProjectModel = body.try_into()?;
    Ok(datamodel)
}

pub fn read_transform_hcl<P: AsRef<Path>>(path: P) -> Result<TransformHCL, ParsingError> {
    let body = load_hcl(path)?;
    let transform_hcl: TransformHCL = body.try_into()?;
    Ok(transform_hcl)
}



#[cfg(test)]
mod test {
    #[test]
    fn test_import() {
        let result =
            super::read_datamodel("../data/testdata/rosetta.hcl");
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
