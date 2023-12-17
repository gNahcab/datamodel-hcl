use std::path::Path;
use std::string::ParseError;
use parse_data::transform_parse::domain::transform_hcl::TransformHCL;

pub fn manipulate_data<P: AsRef<Path>>(data_path: P, data_model_hcl_path: P ,transform_hcl_path: P) -> Result<(), ParseError> {
    //todo: should return the manipulated dataframe
    let project_model = parse_data::operations::read_datamodel(data_model_hcl_path)?;

    let transform_hcl: parse_data::transform_parse::domain::transform_hcl::TransformHCL = parse_data::operations::read_transform_hcl(transform_hcl_path)?;

    if transform_hcl.transform_type == Transfo
    let dataframe: import_data::importers::operations::

    Ok(())
}