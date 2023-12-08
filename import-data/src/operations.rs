use std::path::Path;
use crate::errors::DataImportError;
pub fn load_xlsx<P: AsRef<Path>>(path: P) -> Result<(),DataImportError> {
    let result = crate::importers::xlsx_import::import_xlsx::read_xlsx(path)?;
    Ok(())
}

pub fn load_hcl<P: AsRef<Path>>(path: P) -> Result<hcl::Body, DataImportError> {
    let result = crate::importers::hcl_import::import_hcl::read_hcl_body(path);
    result
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
