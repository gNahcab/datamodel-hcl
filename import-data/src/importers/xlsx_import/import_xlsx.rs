use calamine::{Reader, Xlsx, open_workbook, Range, XlsxError};
use std::path::Path;
use crate::errors::DataImportError;

pub fn read_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<(String, Range<calamine::DataType>)>, DataImportError>{
    let excel: Result<Xlsx<_>, XlsxError> = open_workbook(path);
    match excel {
        Ok(mut excel) => {Ok(excel.worksheets())}
        Err(err) => {
            println!("xlsx-path-problem");
            Err(DataImportError::from(err))
        }
    }
}
