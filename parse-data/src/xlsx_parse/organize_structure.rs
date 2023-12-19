use std::ops::Add;
use std::path::Path;
use calamine::{DataType, Range};
use crate::transform_parse::domain::organized_by::OrganizedBy;
use polars::frame::DataFrame;
use polars::prelude::{NamedFrom, Utf8Type};
use polars::series::Series;
use import_data::errors::DataImportError;
use crate::errors::ParsingError;


pub fn import_some_ordered_df<P: AsRef<Path>>(data_path: P, sheet_numbers: Vec<usize>, organized_bys: Vec<OrganizedBy>) -> Result<Vec<DataFrame>, ParsingError> {
    // import only those sheets that are mentioned in transform-hcl
    let worksheets: Vec<(String, Range<DataType>)> = import_data::operations::load_excel_worksheets(data_path)?;
    let mut all_dataframes:Vec<DataFrame> = vec![];
    let mut nr_ : usize = 0;
    for (i, worksheet) in worksheets.iter().enumerate() {
        if !sheet_numbers.contains(&(i + 1)) {continue;}
        let dataframe = dataframe(worksheet, organized_bys.get(nr_).unwrap())?;
        nr_ += 1;
        all_dataframes.push(dataframe);
    }
    Ok(all_dataframes)

}

pub fn import_all_ordered_df<P: AsRef<Path>>(data_path: P, sheet_numbers: Vec<usize>,  organized_bys: Vec<OrganizedBy>) -> Result<Vec<DataFrame>, ParsingError> {
    let worksheets = import_data::operations::load_excel_worksheets(data_path)?;

    if worksheets.iter().len() != sheet_numbers.iter().len() {
        return Err(ParsingError::ValidationError(format!("all worksheets should be processed but not all worksheets are described, found worksheets in xlsx: '{:?}', worksheets described: '{:?}'",worksheets.iter().len(), sheet_numbers)));
    }
    let mut all_dataframes:Vec<DataFrame> = vec![];
    for (nr_, worksheet) in worksheets.iter().enumerate() {

        let dataframe = dataframe(worksheet, organized_bys.get(nr_).unwrap())?;
        all_dataframes.push(dataframe);
    }

    Ok(all_dataframes)
}
fn dataframe(worksheet: &(String, Range<DataType>), organized_by: &OrganizedBy) -> Result<DataFrame, DataImportError> {
    match organized_by {
        OrganizedBy::ROWOrganized => {
            dataframe_by_row(worksheet)
        }
        OrganizedBy::COLOrganized => {
            dataframe_by_col(worksheet)
        }
    }
}

fn dataframe_by_col(worksheet: &(String, Range<DataType>)) -> Result<DataFrame, DataImportError> {
    // returns a dataframe that is reorganised by column, this is necessary because the importer imports the data by row
    let mut all_series: Vec<Series> = vec![];
    for i in 0..worksheet.1.width() {
        let mut temp_row :Vec<String>= vec![];
            for row in worksheet.1.rows() {
                for (nr, entry) in row.iter().enumerate() {
                    if nr != i { continue }
                    temp_row.push(entry.to_owned().to_string());
                    break
                }
            }
        let s = polars::series::Series::new(i.to_string().as_str(), temp_row);
        all_series.push(s);
    }
    let dataframe = DataFrame::new(all_series)?;
    Ok(dataframe)
}

fn dataframe_by_row(worksheet: &(String, Range<DataType>)) -> Result<DataFrame, DataImportError> {
    let mut all_series: Vec<Series> = vec![];
    for (i, row) in worksheet.1.rows().enumerate(){
        let row_vec: Vec<String> = row.iter().map(|entry|(entry.to_string())).collect();
        let s = polars::series::Series::new(i.to_string().as_str(), row_vec);
        all_series.push(s);
    }
    let dataframe = DataFrame::new(all_series)?;
    Ok(dataframe)
}



