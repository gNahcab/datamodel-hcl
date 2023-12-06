use arrow::*;
use calamine::{Reader, Xlsx, open_workbook};
use std::path::Path;
use crate::errors::DataImportError;
pub fn import_xlsx<P: AsRef<Path>>(path: P) -> Result<(), DataImportError> {

    let mut excel: Xlsx<_> = open_workbook(path)?;
    for worksheet in excel.worksheets() {
        //ignore name of worksheet
        for name in worksheet.1.rows() {
        }
    }
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
    Ok(())

}
//todo: array would be part of ports-directory
fn into_array() -> () {
    //let mut string_array_builder = StringBuilder::new(row_number);
}
#[cfg(test)]
mod test {
    use std::result;
    use crate::adapters::read_xlsx::import_xlsx;

    #[test]
    fn test_sth() {
        let xlsx_path: &str = "../data/testdata/OldExcelDocument.xlsx";
        let result = import_xlsx(xlsx_path);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}



