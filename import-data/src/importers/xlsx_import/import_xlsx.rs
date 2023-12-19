use std::fs::File;
use std::io::BufReader;
use calamine::{Reader, Xlsx, open_workbook, Range, XlsxError};
use std::path::Path;
use polars::prelude::*;
use polars::frame::DataFrame;
use crate::errors::DataImportError;

pub fn read_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<(String, Range<calamine::DataType>)>, DataImportError>{
    let mut excel: Result<Xlsx<_>, XlsxError> = open_workbook(path);
    match excel {
        Ok(mut excel) => {Ok(excel.worksheets())}
        Err(err) => {
            Err(DataImportError::from(err))
        }
    }
}
