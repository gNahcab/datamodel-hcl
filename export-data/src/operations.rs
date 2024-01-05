use std::path::Path;
use manipulate_data::manipulation::shape_data::ShapedData;
use crate::table::WrapperTable;

pub fn export_parquet<P: AsRef<Path>>(data_path: P, data_model_hcl_path: P, transform_hcl_path: P) -> ()  {
    //returns parquet-file
    let shaped_sheets = manipulate_data::operations::process_data(data_path, data_model_hcl_path, transform_hcl_path);
    let shaped_sheets = match shaped_sheets {
        Ok(value) => {value}
        Err(err) => {
            println!("{:?}", err);
            panic!()
        }
    };
    let sample_path = "sample.parquet";
    WrapperTable(shaped_sheets).to_table().to_parquet(sample_path);
}
