use std::path::Path;
use polars::frame::DataFrame;
use crate::errors::DataImportError;
pub fn load_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<DataFrame>,DataImportError> {
    let result = crate::importers::xlsx_import::import_xlsx::read_xlsx(path)?;
    Ok(result)
}

pub fn load_hcl<P: AsRef<Path>>(path: P) -> Result<hcl::Body, DataImportError> {
    let result = crate::importers::hcl_import::import_hcl::read_hcl_body(path)?;
    Ok(result)
}

pub fn load_csv<P: AsRef<Path>>(path: P, delimiter: char) -> Result<DataFrame,DataImportError> {
    let result = crate::importers::csv_import::import_csv::read_csv(path, delimiter)?;
    Ok(result)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_xlsx_import() {
            let result = super::load_xlsx("/Users/gregorbachmann/Documents/Gregor/UniBasel/Masterarbeit/Programmierprojekt/datamodel-hcl/data/testdata/OldExcelDocument.xlsx");
            assert!(result.is_ok());
    }
    #[test]
    fn test_load_hcl() {
        let result = super::load_hcl("/Users/gregorbachmann/Documents/Gregor/UniBasel/Masterarbeit/Programmierprojekt/datamodel-hcl/data/testdata/rosetta.hcl");
        assert!(result.is_ok());
    }
}
