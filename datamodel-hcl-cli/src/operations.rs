use std::path::PathBuf;

pub(crate) fn export_parquet(data_path: &PathBuf, project_path: &PathBuf, transform_path: &PathBuf) {
    let shaped_sheets = manipulate_data::operations::process_data(data_path, project_path,transform_path);
    let shaped_sheets = match shaped_sheets {
        Ok(value) => {value}
        Err(err) => {
            println!("{:?}", err);
            panic!()
        }
    };
    export_data::operations::export_parquet(shaped_sheets);
}
pub(crate) fn export_csv(data_path: &PathBuf, project_path: &PathBuf, transform_path: &PathBuf) {
    let shaped_sheets = manipulate_data::operations::process_data(data_path, project_path,transform_path);
    let shaped_sheets = match shaped_sheets {
        Ok(value) => {value}
        Err(err) => {
            println!("{:?}", err);
            panic!()
        }
    };
    export_data::operations::export_csv(shaped_sheets);
}

pub(crate) fn validate_datamodel(path: &PathBuf) {
    let result = parse_data::operations::read_datamodel(path);
    match result {
        Ok(_) => {
            println!("datamodel is valid");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub(crate) fn validate_transform(path: &PathBuf) {
    let result = parse_data::operations::read_transform_hcl(path);
    match result {
        Ok(_) => {
            println!("transform-hcl is valid");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
