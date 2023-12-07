use calamine::{Reader, Xlsx, open_workbook, Range};
use std::path::Path;
use polars::prelude::*;
use polars::frame::DataFrame;
use crate::errors::DataImportError;
pub fn import_xlsx<P: AsRef<Path>>(path: P) -> Result<(), DataImportError> {

let dataframes: Vec<polars::frame::DataFrame> = get_dataframes(path)?;
Ok(())


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
    let mut all_series: Vec<Series> = vec![];
    for (i, row) in worksheet.1.rows().enumerate(){
        let row_vec: Vec<String> = row.iter().map(|entry| entry.to_string()).collect();
        let s = polars::series::Series::new(i.to_string().as_str(), row_vec);
        all_series.push(s);
    }
    let dataframe = DataFrame::new(all_series)?;
    Ok(dataframe)
}

//todo: array would be part of ports-directory
fn into_array() -> () {
//let mut string_array_builder = StringBuilder::new(row_number);
}
#[cfg(test)]
mod test {
use std::result;
use arrow::buffer;
use arrow::datatypes::ToByteSlice;
use crate::adapters::read_xlsx::import_xlsx;
#[test]
fn test_delete_me() {
    let values: [u8; 12] = [
        b'h', b'e', b'l', b'l', b'o', b'p', b'a', b'r', b'q', b'u', b'e', b't',
    ];
    let offsets: [i32; 4] = [0, 5, 5, 12];

    let array_data = arrow::array::ArrayData::builder(arrow::datatypes::DataType::Utf8)
        .len(3)
        .add_buffer(buffer::Buffer::from(offsets.to_byte_slice()))
        .add_buffer(buffer::Buffer::from(&values[..]))
        .null_bit_buffer(Some(buffer::Buffer::from([0b00000101])))
        .build()
        .unwrap();
    let binary_array = arrow::array::StringArray::from(array_data);
    println!("{binary_array:?}");
}
#[test]
fn test_sth() {
    let xlsx_path: &str = "../data/testdata/OldExcelDocument.xlsx";
    let result = import_xlsx(xlsx_path);
    println!("{:?}", result);
    assert!(result.is_ok());
}
}



