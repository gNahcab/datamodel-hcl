use std::path::{Path, PathBuf};
use polars::prelude::*;
use crate::errors::DataImportError;

pub(crate) fn read_csv<P: AsRef<Path>>(path: P, delimiter: char) -> Result<DataFrame, DataImportError> where PathBuf: From<P> {
    let df = CsvReader::from_path(path)
        .unwrap().has_header(false).with_delimiter(u8::try_from(delimiter).unwrap())
        .finish()
        .unwrap();
    Ok(df)
}

