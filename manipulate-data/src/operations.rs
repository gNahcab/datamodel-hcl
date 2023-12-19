use std::path::Path;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::transform_hcl::TransformHCL;
use parse_data::transform_parse::domain::transform_type::TransformType;
use crate::manipulation;

pub fn manipulate_data<P: AsRef<Path>>(data_path: P, data_model_hcl_path: P ,transform_hcl_path: P) -> Result<(), ParsingError> {
    //todo: should return the manipulated dataframe as polars dataframe or somethin
    let project_model = parse_data::operations::read_datamodel(data_model_hcl_path)?;
    let transform_hcl: parse_data::transform_parse::domain::transform_hcl::TransformHCL = parse_data::operations::read_transform_hcl(transform_hcl_path)?;
    match transform_hcl.transform_type {

                TransformType::XLSX(transform_xlsx) => {
                    let dataframe = manipulation::manipulate::manipulate_xlsx_data(transform_xlsx, data_path);
                }
                TransformType::CSV(transform_csv) => {
                    let dataframe = manipulation::manipulate::manipulate_csv_data(transform_csv, data_path);

                }
            }

    Ok(())
}
#[cfg(test)]
mod test {
    use crate::manipulation::manipulate::manipulate_xlsx_data;
    use crate::operations::manipulate_data;

    #[test]
    fn test_manipulate_xlsx() {
        let transform_path = "../data/testdata/transform_xlsx.hcl";
        let xlsx_path = "../data/testdata/col_test.xlsx";
        let data_model_path = "../data/testdata/rosetta.hcl";
        let result = manipulate_data(xlsx_path,data_model_path,transform_path);
        println!("test_res: {:?}", result);
        assert!(result.is_ok())
    }
}
