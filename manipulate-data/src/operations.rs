use std::path::Path;
use polars::frame::DataFrame;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::transform_type::TransformType;
use crate::manipulation;
use crate::manipulation::manipulate::{process_csv_data, process_xlsx_data};
use crate::manipulation::shape_data::ShapedData;

pub fn process_data<P: AsRef<Path>>(data_path: P, data_model_hcl_path: P, transform_hcl_path: P) -> Result<Vec<ShapedData>, ParsingError> {
    let project_model = parse_data::operations::read_datamodel(data_model_hcl_path)?;
    let transform_hcl: parse_data::transform_parse::domain::transform_hcl::TransformHCL = parse_data::operations::read_transform_hcl(transform_hcl_path)?;
    let data_sheets = match transform_hcl.transform_type {
        TransformType::XLSX(transform_xlsx) => {
            process_xlsx_data(transform_xlsx, data_path)?
        }
        TransformType::CSV(transform_csv) => {
            process_csv_data(transform_csv, data_path)?
        }
    };
    let final_sheets  = manipulation::shape_data::shape_with_data_model(data_sheets, project_model)?;
    Ok(final_sheets)
}

//#[pymodule]


#[cfg(test)]
mod test {
    use crate::operations::process_data;

    #[test]
    fn test_manipulate_xlsx() {
        let transform_path = "../data/testdata/transform_xlsx.hcl";
        let xlsx_path = "../data/testdata/test_file_xlsx_col.xlsx";
        let data_model_path = "../data/testdata/rosetta.hcl";
        let result = process_data(xlsx_path, data_model_path, transform_path);
        println!("test_res: {:?}", result);
        assert!(result.is_ok())
    }
}
