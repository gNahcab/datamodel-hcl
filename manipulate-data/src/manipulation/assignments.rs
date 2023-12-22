use polars::frame::DataFrame;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::sheet_info::SheetInfo;
use parse_data::transform_parse::domain::transform_type::TransformXLSX;
use parse_data::xlsx_parse::data_sheet::DataSheet;

pub(crate) fn add_assignments_xlsx(mut data_sheets: Vec<DataSheet>, transform_xlsx: &TransformXLSX) -> Result<Vec<DataSheet>, ParsingError> {
    for (i, data_sheet) in data_sheets.iter().enumerate() {
       let sheet_info =  match transform_xlsx.worksheets.get((i)) {
            None => {
                return Err(ParsingError::ValidationError(format!("tried to get worksheet nr '{:?}' from worksheets, but it wasn't there. This should never happen.", i)))
            }
            Some(sheet_info) => {
                sheet_info
            }
        };

        data_sheet.add_assignment(sheet_info)?;
    }
    Ok(data_sheets)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use parse_data::transform_parse::domain::assignment::Assignments;
    use parse_data::transform_parse::domain::header_value::HeaderValue;
    use parse_data::transform_parse::domain::organized_by::OrganizedBy;
    use parse_data::transform_parse::domain::sheet_info::SheetInfo;
    use parse_data::transform_parse::domain::transform_type::TransformXLSX;
    use parse_data::xlsx_parse::data_sheet::DataSheet;
    use crate::manipulation::assignments::add_assignments_xlsx;
    #[test]
    fn test_add_assignments_xlsx_numbers() {
        let mut data_sheet: DataSheet = DataSheet::new();
        data_sheet.add_height(5);
        data_sheet.add_width(4);
        let row:Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec!["aa".to_string(), "bb".to_string(), "cc".to_string(), "dd".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string(), "ddd".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec!["aaaa".to_string(), "bbbb".to_string(), "cccc".to_string(), "dddd".to_string()];
        data_sheet.tabular_data.push(row);
        let mut assignments: HashMap<String, HeaderValue> = HashMap::new();
        assignments.insert("Label".to_string(), HeaderValue::Number(1));
        assignments.insert("hasValue".to_string(), HeaderValue::Number(2));
        assignments.insert("hasExternalLink".to_string(),  HeaderValue::Number(3));
        assignments.insert("ID".to_string(), HeaderValue::Number(4));
        let transform_xlsx =  TransformXLSX{
            all_sheets: true,
            sheet_numbers: vec![],
            organized_bys: vec![OrganizedBy::ROWOrganized],
            worksheets: vec![SheetInfo{
                sheet_nr: 1,
                structured_by: OrganizedBy::ROWOrganized,
                headers_exist: false,
                resource: None,
                resource_row: None,
                assignments: Assignments {assignments_to_header_value:assignments},
                transformations: None,
            }],
        };
        let result = add_assignments_xlsx(vec![data_sheet], &transform_xlsx);
        assert!(result.is_ok());

    }
    #[test]
    fn test_add_assignments_xlsx_headers() {
        let mut data_sheet: DataSheet = DataSheet::new();
        data_sheet.add_height(5);
        data_sheet.add_width(4);
        let row:Vec<String> = vec!["names_column".to_string(), "values_column".to_string(), "links_column".to_string(), "all_ids".to_string()];
        data_sheet.headers = Option::from(row);
        let row:Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec!["aa".to_string(), "bb".to_string(), "cc".to_string(), "dd".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string(), "ddd".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec!["aaaa".to_string(), "bbbb".to_string(), "cccc".to_string(), "dddd".to_string()];
        data_sheet.tabular_data.push(row);
        let mut assignments: HashMap<String, HeaderValue> = HashMap::new();
        assignments.insert("Label".to_string(), HeaderValue::Name("names_column".to_string()));
       assignments.insert("hasValue".to_string(), HeaderValue::Name("values_column".to_string()));
       assignments.insert("hasExternalLink".to_string(), HeaderValue::Name("links_column".to_string()));
       assignments.insert("ID".to_string(), HeaderValue::Name("all_ids".to_string()));
       let transform_xlsx =  TransformXLSX{
            all_sheets: true,
            sheet_numbers: vec![],
            organized_bys: vec![OrganizedBy::ROWOrganized],
            worksheets: vec![SheetInfo{
                sheet_nr: 1,
                structured_by: OrganizedBy::ROWOrganized,
                headers_exist: true,
                resource: None,
                resource_row: None,
                assignments: Assignments {assignments_to_header_value:assignments},
                transformations: None,
            }],
        };
        let result = add_assignments_xlsx(vec![data_sheet], &transform_xlsx);
        println!("result {:?}", result);
        assert!(result.is_ok());

    }
}
