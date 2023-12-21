use std::any::Any;
use std::collections::HashMap;
use crate::errors::ParsingError;
use crate::transform_parse::domain::assignment::Assignments;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_hcl::{TransformHCL, TransientStructureTransformHCL};
use crate::transform_parse::domain::transform_type::{TransformCSV, TransformType, TransformXLSX};

pub struct TransformHCLBuilder {
    transform: String,
    all_sheets: bool,
    sheets: Vec<usize>,
    organized_bys: Vec<OrganizedBy>,
    worksheets: HashMap<usize, SheetInfo>,
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
            worksheets: transient_structure.worksheets,
        }
    }
    fn is_consistent(&self) {
        todo!()
        // check sheets
        // check worksheets
        // check csv-transform or xlsx-transform is correct
    }
    fn is_xlsx_consistent(&self) -> Result<(), ParsingError>{
        // check numbers of xlsx and sheet_numbers fit (even if all_sheets=true)
        // check that if headers_exist is false no assignments to a header were made
        // check worksheets in between are correct?
        // check csv-transform or xlsx-transform is correct
        Ok(())
    }
    fn is_csv_consistent(&self) -> Result<(), ParsingError>{
        todo!()
        // check sheets
        // check worksheets
        // check csv-transform or xlsx-transform is correct
    }
    pub fn build(&self) -> Result<TransformHCL, ParsingError> {
        println!("transform::: {:?}", self.transform);
        match self.transform.as_str() {
            "csv" => {
                self.is_csv_consistent()?;
                todo!()
            }
           "xlsx" => {
               self.is_xlsx_consistent()?;

               let worksheets: Vec<SheetInfo> = self.worksheets.iter().map(|(sheet_nr, sheet_info)|sheet_info.to_owned()).collect();
               Ok(TransformHCL { transform_type: TransformType::XLSX(TransformXLSX { all_sheets: self.all_sheets, sheet_numbers: self.sheets.to_owned(), organized_bys: self.organized_bys.to_owned(), worksheets: worksheets})})
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
