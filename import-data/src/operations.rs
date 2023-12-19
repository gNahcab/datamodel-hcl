use std::path::{Path, PathBuf};
use calamine::Range;
use polars::frame::DataFrame;
use crate::errors::DataImportError;
use crate::importers::xlsx_import::import_xlsx::read_xlsx;


pub fn load_excel_worksheets<P: AsRef<Path>>(path: P) -> Result<Vec<(String, Range<calamine::DataType>)>, DataImportError> {
    read_xlsx(path)
}

pub fn load_hcl<P: AsRef<Path>>(path: P) -> Result<hcl::Body, DataImportError> {
    let result = crate::importers::hcl_import::import_hcl::read_hcl_body(path)?;
    Ok(result)
}

pub fn load_csv_dataframe<P: AsRef<Path>>(path: P, delimiter: char) -> Result<DataFrame,DataImportError> where PathBuf: From<P>{
    let result = crate::importers::csv_import::import_csv::read_csv(path, delimiter)?;
    Ok(result)
}


#[cfg(test)]
mod test {
    #[test]
    fn test_xlsx_import() {
            let vec:Vec<usize> = vec![1];
            let result = super::load_excel_worksheets("../data/testdata/OldExcelDocument.xlsx");
            assert!(result.is_ok());
    }
    #[test]
    fn test_load_hcl() {
        let result = super::load_hcl("../data/testdata/rosetta.hcl");
        assert!(result.is_ok());
    }
}
