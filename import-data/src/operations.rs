use std::path::Path;
use crate::errors::DataImportError;
pub fn load_xlsx<P: AsRef<Path>>(path: P) -> Result<(),DataImportError> {
    let result = crate::importers::xlsx_import::read_xlsx::read_xlsx(path)?;
    Ok(())
}

pub fn load_hcl<P: AsRef<Path>>(path: P) -> Result<(),DataImportError> {
    let result = crate::importers::hcl_import::read_transform_hcl::read_transform_hcl(path)?;
    Ok(())
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
        let result = super::load_hcl("/Users/gregorbachmann/Documents/Gregor/UniBasel/Masterarbeit/Programmierprojekt/datamodel-hcl/data/testdata/transform_hcl.hcl");
        assert!(result.is_ok());
    }
}
