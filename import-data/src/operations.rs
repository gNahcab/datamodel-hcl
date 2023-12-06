use std::path::Path;
use crate::errors::DataImportError;
pub fn load_xlsx<P: AsRef<Path>>(path: P) -> Result<(),DataImportError> {
    let result = crate::adapters::read_xlsx::import_xlsx(path)?;
    Ok(())
}


#[cfg(test)]
mod test {
    #[test]
    fn test_xlsx_import() {
            println!("hi");
            let result = super::load_xlsx("/Users/gregorbachmann/Documents/Gregor/UniBasel/Masterarbeit/Programmierprojekt/datamodel-hcl/data/testdata/OldExcelDocument.xlsx");
            println!("result was: {:?}", result);
            assert!(result.is_ok());
    }
}
