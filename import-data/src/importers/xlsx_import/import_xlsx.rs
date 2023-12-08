use calamine::{Reader, Xlsx, open_workbook, Range};
use std::path::Path;
use polars::prelude::*;
use polars::frame::DataFrame;
use crate::errors::DataImportError;
pub fn read_xlsx<P: AsRef<Path>>(path: P) -> Result<Vec<DataFrame>, DataImportError> {
    // todo change method naming: get replace with correct vocabulary for methods, see: https://rust-lang.github.io/api-guidelines/naming.html
let dataframes: Vec<polars::frame::DataFrame> = get_dataframes(path)?;
Ok((dataframes))
}

fn get_dataframes<P: AsRef<Path>>(path: P) -> Result<Vec<DataFrame>, DataImportError> {
    let mut excel: Xlsx<_> = open_workbook(path)?;
    let mut all_dataframes = vec![];
    for worksheet in excel.worksheets() {
        let dataframe = get_dataframe(worksheet)?;
        all_dataframes.push(dataframe);
    }
    return Ok(all_dataframes);
}

fn get_dataframe(worksheet: (String, Range<calamine::DataType>)) -> Result<DataFrame, DataImportError> {
    // todo change method-name: to_dataframe?
    let mut all_series: Vec<Series> = vec![];
    for (i, row) in worksheet.1.rows().enumerate(){
        let row_vec: Vec<String> = row.iter().map(|entry| entry.to_string()).collect();
        let s = polars::series::Series::new(i.to_string().as_str(), row_vec);
        all_series.push(s);
    }
    let dataframe = DataFrame::new(all_series)?;
    Ok(dataframe)
}




