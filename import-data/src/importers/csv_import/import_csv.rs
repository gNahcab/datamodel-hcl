use std::path::{Path, PathBuf};
use std::string::ParseError;
use polars::prelude::*;
pub(crate) fn read_csv<P: AsRef<Path>>(path: P, delimiter: char) -> Result<DataFrame, ParseError> where PathBuf: From<P> {
    let df = CsvReader::from_path(path)
        .unwrap().has_header(false).with_delimiter(u8::try_from(delimiter).unwrap())
        .finish()
        .unwrap();
    Ok(df)
}
#[cfg(test)]
mod test {
    use crate::importers::csv_import::import_csv::read_csv;
    use crate::importers::xlsx_import::import_xlsx::read_xlsx;

    #[test]
    fn test_read_csv() {
        //read_csv("../data/testdata/test.csv", ';');
        let _ = read_xlsx("../data/testdata/OldExcelDocument.xlsx");
    }
    }
