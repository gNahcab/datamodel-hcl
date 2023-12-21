use polars::frame::DataFrame;
use parse_data::transform_parse::domain::transform_type::TransformXLSX;
use parse_data::xlsx_parse::data_sheet::DataSheet;

pub(crate) fn add_assignments_xlsx(data_sheets: Vec<DataSheet>, transform_xlsx: &TransformXLSX) -> () {
    println!("transform: {:?}", transform_xlsx);
}
