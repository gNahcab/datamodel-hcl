use std::any::Any;
use std::collections::HashSet;
use std::hash::Hash;
use crate::errors::ParsingError;
use crate::transform_parse::domain::header_value::HeaderValue;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_hcl::{TransformHCL, TransientStructureTransformHCL};
use crate::transform_parse::domain::transform_type::{TransformCSV, TransformType, TransformXLSX};

pub struct TransformHCLBuilder {
    transform: String,
    all_sheets: bool,
    sheets: Vec<usize>,
    organized_bys: Vec<OrganizedBy>,
    worksheets: Vec<SheetInfo>
}

impl TransformHCLBuilder {
    pub fn new(transient_structure: TransientStructureTransformHCL) -> TransformHCLBuilder {
        let all_sheets = match transient_structure.all_sheets {
            None => {false}
            Some(value) => {value}
        };
        TransformHCLBuilder{
            transform: transient_structure.transform.unwrap(),
            all_sheets,
            sheets: transient_structure.sheets ,
            organized_bys: transient_structure.organized_bys,
            worksheets: transient_structure.worksheets
        }
    }
    fn is_consistent(&self) {
        todo!()
        // check sheets
        // check worksheets
        // check csv-transform or xlsx-transform is correct
    }

    fn is_xlsx_consistent(&self) -> Result<(), ParsingError>{
        let all_numbers: Vec<&usize> = self.worksheets.iter().map(|sheet_info|&sheet_info.sheet_nr).collect();
        let mut uniq = HashSet::new();
        for number in all_numbers.iter() {
            // worksheets: no sheet-number more than once used
            if uniq.insert(number) == false {
                return Err(ParsingError::ValidationError(format!("found duplicate sheet-number: '{:?}'. Every sheet number must be unique.", number)));
            }
            // worksheets in sheets?
            if self.all_sheets == false {
                if !self.sheets.contains(&number)
                {
                    return Err(ParsingError::ValidationError(format!("sheet-number is not mentioned in 'sheets'. missing number: '{:?}', the current 'sheets'-numbers: '{:?}'. Every sheet number of a sheet must be mentioned in 'sheets'.", number, self.sheets)));
                }
            }

        }
        // sheets in worksheets?
        if self.all_sheets == false {
            let missing_sheet_numbers: Vec<&usize> = self.sheets.iter().filter(|sheet_nr|!all_numbers.contains(sheet_nr)).collect();
            if missing_sheet_numbers.len() != 0 {
                return Err(ParsingError::ValidationError(format!("some numbers in 'sheet' are not used in described sheets: '{:?}'. Every sheet number mentioned in 'sheets' must be assigned to a sheet.", missing_sheet_numbers)));
            }
        }
        // check that if headers_exist is false no resource_row with a header exists
        for sheet_info in self.worksheets.iter() {
            if sheet_info.headers_exist == true {
                continue
            }
            if sheet_info.resource_row.is_some() == true {
                match &sheet_info.resource_row.as_ref().unwrap() {
                    HeaderValue::Name(value) => {
                        return Err(ParsingError::ValidationError(format!("resource_row in sheet-nr '{:?}' contains a string as header '{:?}', but 'headers' is set to false.", sheet_info.sheet_nr, value)));
                    }
                    HeaderValue::Number(_) => {
                        //ok..
                    }
                }
            }
            // check that if headers_exist is false no assignments to a header were made
            let string_headers: Vec<&String> = sheet_info.assignments.assignments_to_header_value.iter().filter_map(|(assignment, header_value)| match header_value {
                HeaderValue::Name(name) => Some(name),
                _ => None,
            }).collect();
            if string_headers.len() != 0 {
                return Err(ParsingError::ValidationError(format!("'assignments' in sheet-nr '{:?}' contains strings as headers '{:?}', but 'headers' is set to false.", sheet_info.sheet_nr, string_headers)));
            }
        }
        Ok(())
    }
    fn is_csv_consistent(&self) -> Result<(), ParsingError>{
        todo!()
        // check sheets
        // check worksheets
        // check csv-transform or xlsx-transform is correct
    }
    pub fn build(&mut self) -> Result<TransformHCL, ParsingError> {

        match self.transform.as_str() {
            "csv" => {
                self.is_csv_consistent()?;
                todo!()
            }
           "xlsx" => {
               // sort worksheets by sheet-number, without sort: cannot compare to xlsx-sheets (which are sorted by sheet-number)
               self.worksheets.sort_by(|sheet_a, sheet_b| sheet_a.sheet_nr.cmp(&sheet_b.sheet_nr));
               self.is_xlsx_consistent()?;
               Ok(TransformHCL { transform_type: TransformType::XLSX(TransformXLSX { all_sheets: self.all_sheets, sheet_numbers: self.sheets.to_owned(), organized_bys: self.organized_bys.to_owned(), worksheets: self.worksheets.iter().map(|sheet_info|sheet_info.to_owned()).collect()})})
            }
            _ => {
                Err(ParsingError::ValidationError(format!("cannot parse 'transform'-expression: '{:?}'. Only 'xslx' and 'csv' are valid", self.transform)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_builder() {
        todo!()
        }
}
