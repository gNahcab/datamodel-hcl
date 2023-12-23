use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::sheet_info::SheetInfo;
use parse_data::transform_parse::domain::transform_type::TransformXLSX;
use parse_data::transform_parse::domain::transformations::Transformations;
use parse_data::xlsx_parse::parsed_data_sheet::ParsedDataSheet;


pub fn check_consistency(data_sheets: &Vec<ParsedDataSheet>, transform_xlsx: &TransformXLSX) -> Result<(), ParsingError> {
    for (i, data_sheet) in data_sheets.iter().enumerate() {
        let sheet_info = transform_xlsx.worksheets.get(i).unwrap();
        data_sheet.check_assignments_from_sheet_info(sheet_info)?;
        data_sheet.check_transform_form_sheet_info(sheet_info)?;
        println!("1");
    }
    Ok(())
}
pub fn manipulate_xlsx_data_sheets(data_sheets: Vec<ParsedDataSheet>, transform_xlsx: &TransformXLSX) -> Result<(), ParsingError> {
    let mut new_data_sheets: Vec<ParsedDataSheet> = vec![];
    for (i, data_sheet) in data_sheets.iter().enumerate() {
        let sheet_info: &SheetInfo = transform_xlsx.worksheets.get(i).unwrap();
        if sheet_info.transformations.is_none() {
            //clone old one and continue
            let new_data_sheet = data_sheet.copy();
            new_data_sheets.push(new_data_sheet);
            continue
        }
        let new_data_sheet = transform_data_sheet(data_sheet, &sheet_info.transformations.as_ref().unwrap())?;
        new_data_sheets.push(new_data_sheet);
    }
    Ok(())
}

fn transform_data_sheet(data_sheet: &ParsedDataSheet, transformations: &Transformations) -> Result<ParsedDataSheet, ParsingError> {
    println!("transformations:: {:?}", transformations);


    Ok(ParsedDataSheet::new())
}

pub(crate) fn add_assignments_xlsx(data_sheets: Vec<ParsedDataSheet>, transform_xlsx: &TransformXLSX) -> Result<Vec<ParsedDataSheet>, ParsingError> {
    let mut i = 0;
    let mut new_data_sheets: Vec<ParsedDataSheet> = vec![];
    for mut data_sheet in data_sheets {
       let sheet_info =  transform_xlsx.worksheets.get((i)).unwrap();
        i+= 1;
        let mut new_data_sheet = data_sheet;
        new_data_sheet.check_assignments_from_sheet_info(sheet_info)?;
        new_data_sheets.push(new_data_sheet);
    }
    Ok(new_data_sheets)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use parse_data::transform_parse::domain::assignment::Assignments;
    use parse_data::transform_parse::domain::header_value::HeaderValue;
    use parse_data::transform_parse::domain::methods_domain::behavior_type::BehaviorType;
    use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
    use parse_data::transform_parse::domain::methods_domain::target_type::TargetType;
    use parse_data::transform_parse::domain::organized_by::OrganizedBy;
    use parse_data::transform_parse::domain::sheet_info::SheetInfo;
    use parse_data::transform_parse::domain::transform_type::TransformXLSX;
    use parse_data::transform_parse::domain::transformations::Transformations;
    use parse_data::xlsx_parse::parsed_data_sheet::ParsedDataSheet;
    use crate::manipulation::xlsx_data_sheet::{add_assignments_xlsx, manipulate_xlsx_data_sheets, transform_data_sheet};

    #[test]
    fn test_check_transform() {
        let mut data_sheet: ParsedDataSheet = ParsedDataSheet::new();
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

        let lower_method = parse_data::transform_parse::domain::methods_domain::lower_upper_method::LowerMethod{ output: "hasLowerValue".to_string(), input: HeaderValue::Number(0) };
        let upper_method = parse_data::transform_parse::domain::methods_domain::lower_upper_method::UpperMethod{ output: "hasUpperValue".to_string(), input: HeaderValue::Name("hasValue".to_string())  };
        let replace_method = parse_data::transform_parse::domain::methods_domain::replace_method::ReplaceMethod{
            output: "hasExternalLink2".to_string(),
            input: HeaderValue::Name("hasExternalLink".to_string()),
            replace: vec!["http".to_string(), "https".to_string()],
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let combine_method = parse_data::transform_parse::domain::methods_domain::combine_method::CombineMethod{
            input: vec![HeaderValue::Name("names_column".to_string()), HeaderValue::Number(4)],
            output: "hasNewValue".to_string(),
            separator: Option::from("_".to_string()),
            prefix: Option::from("my_project".to_string()),
            suffix: None,
        };
        let to_date_method = parse_data::transform_parse::domain::methods_domain::to_date_method::ToDateMethod{
            output: "hasDate".to_string(),
            input: HeaderValue::Name("hasValue".to_string()),
            date_type: DateType::Gregorian,
        };
        let transformations = Transformations{
            lower_methods: vec![lower_method],
            upper_methods: vec![upper_method],
            combine_methods: vec![combine_method],
            replace_methods: vec![replace_method],
            to_date_methods: vec![to_date_method],
        };
        let sheet_info = SheetInfo{
                sheet_nr: 1,
                structured_by: OrganizedBy::ROWOrganized,
                headers_exist: true,
                resource: None,
                resource_row: Option::from(HeaderValue::Number(2)),
                assignments: Assignments {assignments_to_header_value:assignments},
                transformations: Option::from(transformations),
            };
        let result = data_sheet.check_transform_form_sheet_info(&sheet_info);
        println!("result {:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn test_add_transform() {
        let mut data_sheet: ParsedDataSheet = ParsedDataSheet::new();
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
        let lower_method = parse_data::transform_parse::domain::methods_domain::lower_upper_method::LowerMethod{ output: "hasLowerValue".to_string(), input: HeaderValue::Number(0) };
        let upper_method = parse_data::transform_parse::domain::methods_domain::lower_upper_method::UpperMethod{ output: "hasUpperValue".to_string(), input: HeaderValue::Name("hasValue".to_string())  };
        let replace_method = parse_data::transform_parse::domain::methods_domain::replace_method::ReplaceMethod{
            output: "hasExternalLink2".to_string(),
            input: HeaderValue::Name("hasExternalLink".to_string()),
            replace: vec!["http".to_string(), "https".to_string()],
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let combine_method = parse_data::transform_parse::domain::methods_domain::combine_method::CombineMethod{
            input: vec![HeaderValue::Name("names_column".to_string()), HeaderValue::Number(4)],
            output: "hasNewValue".to_string(),
            separator: Option::from("_".to_string()),
            prefix: Option::from("my_project".to_string()),
            suffix: None,
        };
        let to_date_method = parse_data::transform_parse::domain::methods_domain::to_date_method::ToDateMethod{
            output: "hasDate".to_string(),
            input: HeaderValue::Name("hasValue".to_string()),
            date_type: DateType::Gregorian,
        };
        let transformations = Transformations{
            lower_methods: vec![lower_method],
            upper_methods: vec![upper_method],
            combine_methods: vec![combine_method],
            replace_methods: vec![replace_method],
            to_date_methods: vec![to_date_method],
        };
        let result = transform_data_sheet(&data_sheet, &transformations);
        println!("result {:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn test_check_assignments_xlsx_numbers() {
        let mut data_sheet: ParsedDataSheet = ParsedDataSheet::new();
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
        let result = data_sheet.check_assignments_from_sheet_info(transform_xlsx.worksheets.get(0).unwrap());
        assert!(result.is_ok());

    }
    #[test]
    fn test_check_assignments_xlsx_headers() {
        let mut data_sheet: ParsedDataSheet = ParsedDataSheet::new();
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
                resource_row: Option::from(HeaderValue::Number(2)),
                assignments: Assignments {assignments_to_header_value:assignments},
                transformations: None,
            }],
        };
        let result = data_sheet.check_assignments_from_sheet_info(transform_xlsx.worksheets.get(0).unwrap());
        println!("result {:?}", result);
        assert!(result.is_ok());

    }
}
