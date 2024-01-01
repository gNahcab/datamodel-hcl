use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::sheet_info::SheetInfo;
use parse_data::transform_parse::domain::transform_type::TransformXLSX;
use parse_data::transform_parse::domain::transformations::Transformations;
use parse_data::xlsx_parse::data_sheet::DataSheet;
use crate::manipulation::manipulated_data_sheet::{ManipulatedDataSheet, ManipulatedDataSheetWrapper};


pub fn check_consistency(data_sheets: &Vec<DataSheet>, transform_xlsx: &TransformXLSX) -> Result<(), ParsingError> {
    for (i, data_sheet) in data_sheets.iter().enumerate() {
        let sheet_info = transform_xlsx.worksheets.get(i).unwrap();
        data_sheet.check_assignments_from_sheet_info(sheet_info)?;
        data_sheet.check_transform_form_sheet_info(sheet_info)?;
    }
    Ok(())
}
pub fn manipulate_xlsx_data_sheets(data_sheets: Vec<DataSheet>, transform_xlsx: &TransformXLSX) -> Result<Vec<ManipulatedDataSheet>, ParsingError> {
    let mut new_data_sheets: Vec<ManipulatedDataSheet> = vec![];
    for (i, data_sheet) in data_sheets.iter().enumerate() {
        let sheet_info: &SheetInfo = transform_xlsx.worksheets.get(i).unwrap();
        let new_data_sheet = transform_data_sheet(data_sheet, &sheet_info.transformations)?;
        new_data_sheets.push(new_data_sheet);
    }
    Ok(new_data_sheets)
}

fn transform_data_sheet(data_sheet: &DataSheet, transformations: &Option<Transformations>) -> Result<ManipulatedDataSheet, ParsingError> {
                    ManipulatedDataSheetWrapper(data_sheet.copy(), transformations.to_owned()).to_manipulated_data_sheet()
}

pub(crate) fn add_assignments_xlsx(data_sheets: Vec<DataSheet>, transform_xlsx: &TransformXLSX) -> Result<Vec<DataSheet>, ParsingError> {
    let mut i = 0;
    let mut new_data_sheets: Vec<DataSheet> = vec![];
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
    use parse_data::transform_parse::domain::methods_domain::combine_method::CombineMethod;
    use parse_data::transform_parse::domain::methods_domain::date_bricks::{DateBricks, DateInfo, DateName};
    use parse_data::transform_parse::domain::methods_domain::date_pattern::DatePattern;
    use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
    use parse_data::transform_parse::domain::methods_domain::target_type::TargetType;
    use parse_data::transform_parse::domain::organized_by::OrganizedBy;
    use parse_data::transform_parse::domain::sheet_info::SheetInfo;
    use parse_data::transform_parse::domain::transform_type::TransformXLSX;
    use parse_data::transform_parse::domain::transformations::Transformations;
    use parse_data::xlsx_parse::data_sheet::DataSheet;
    use crate::manipulation::xlsx_data_sheet::transform_data_sheet;

    #[test]
    fn test_check_transform() {
        let mut data_sheet: DataSheet = DataSheet{
            tabular_data: vec![],
            height: 5,
            width: 4,
            headers: vec![],
            assignments: Default::default(),
        };
        let row:Vec<String> = vec!["names_column".to_string(), "values_column".to_string(), "links_column".to_string(), "all_ids".to_string()];
        data_sheet.headers = row;
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
            old: "http".to_string(),
            new: "https".to_string(),
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
            date_patterns: [DatePattern{
                nr: 1,
                first_date: None,
                date: DateBricks{
                    month_word: Option::from(false),
                    day: Option::from(DateInfo { nr: 1, name: DateName::Day }),
                    month: Option::from(DateInfo { nr: 2, name: DateName::Month }),
                    year: Option::from(DateInfo { nr: 3, name: DateName::Year }),
                },
            }].to_vec(),
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
    fn test_perform_transform() {
        let mut assignments: HashMap<String, HeaderValue> = HashMap::new();
        assignments.insert("Label".to_string(), HeaderValue::Name("names_column".to_string()));
        assignments.insert("all_dates".to_string(), HeaderValue::Name("my_dates_column".to_string()));
        assignments.insert("hasExternalLink".to_string(), HeaderValue::Name("links_column".to_string()));
        assignments.insert("ID".to_string(), HeaderValue::Name("all_ids".to_string()));
        let mut data_sheet: DataSheet = DataSheet{
            tabular_data: vec![],
            height: 5,
            width: 4,
            headers: vec![],
            assignments: assignments,
        };
        let headers: Vec<String> = vec!["names_column".to_string(),"my_dates_column".to_string(),"links_column".to_string(),"all_ids".to_string()];
        data_sheet.headers = headers;

        let row:Vec<String> = vec![ "BM K.3375".to_string(), "Deluge".to_string(), "Liberté, j'écris ton nom".to_string(),"Bashō, Horohoroto".to_string(),];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec![ "700-600".to_string(), "1877-1879".to_string(), "1953".to_string(), "1688".to_string(),];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec![ "inst_0".to_string(), "inst_1".to_string(), "inst_2".to_string(), "inst_3".to_string()];
        data_sheet.tabular_data.push(row);
        let row:Vec<String> = vec![ "img_obj_0".to_string(), "img_obj_1".to_string(), "img_obj_2".to_string(), "img_obj_3".to_string()];
        data_sheet.tabular_data.push(row);

        let lower_method = parse_data::transform_parse::domain::methods_domain::lower_upper_method::LowerMethod{ output: "hasLowerValue".to_string(), input: HeaderValue::Number(0) };
        let upper_method = parse_data::transform_parse::domain::methods_domain::lower_upper_method::UpperMethod{ output: "hasUpperValue".to_string(), input: HeaderValue::Name("my_dates_column".to_string())  };
        let replace_method = parse_data::transform_parse::domain::methods_domain::replace_method::ReplaceMethod{
            output: "hasExternalLink2".to_string(),
            input: HeaderValue::Name("hasExternalLink".to_string()),
            old: "http".to_string(),
            new: "https".to_string(),
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let inputs = [HeaderValue::Name("Label".to_string()),HeaderValue::Name("ID".to_string())];
        let combine_method: CombineMethod  = CombineMethod{
            input: inputs.to_vec(),
            output: "hasNewID".to_string(),
            separator: Option::from("_".to_string()),
            prefix: Option::from("rosetta_".to_string()),
            suffix: Option::from("_version.1".to_string()),
        };
        let to_date_method = parse_data::transform_parse::domain::methods_domain::to_date_method::ToDateMethod{
            output: "hasDate".to_string(),
            input: HeaderValue::Name("all_dates".to_string()),
            date_type: DateType::Gregorian,
            date_patterns: [
                DatePattern{
                nr: 1,
                first_date: None,
                date: DateBricks{
                    month_word: Option::from(false),
                    day: Option::from(DateInfo { nr: 1, name: DateName::Day }),
                    month: Option::from(DateInfo { nr: 2, name: DateName::Month }),
                    year: Option::from(DateInfo { nr: 3, name: DateName::Year }),
                },
            },   DatePattern{
                    nr: 2,
                    first_date: None,
                    date: DateBricks{
                        month_word: Option::from(false),
                        day: None,
                        month: None,
                        year: Option::from(DateInfo { nr: 3, name: DateName::Year }),
                    },
                },
            ].to_vec(),
        };
        let transformations = Transformations{
            lower_methods: vec![lower_method],
            upper_methods: vec![upper_method],
            combine_methods: vec![combine_method],
            replace_methods: vec![replace_method],
            to_date_methods: vec![to_date_method],
        };
        let result = transform_data_sheet(&data_sheet, &Option::from(transformations));
        println!("result {:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn test_check_assignments_xlsx_numbers() {
        let mut data_sheet: DataSheet = DataSheet{
            tabular_data: vec![],
            height: 5,
            width: 4,
            headers: vec![],
            assignments: Default::default(),
        };
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
        let mut data_sheet: DataSheet = DataSheet{
            tabular_data: vec![],
            height: 5,
            width: 4,
            headers: vec![],
            assignments: Default::default(),
        };
        let row:Vec<String> = vec!["names_column".to_string(), "values_column".to_string(), "links_column".to_string(), "all_ids".to_string()];
        data_sheet.headers = row;
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
