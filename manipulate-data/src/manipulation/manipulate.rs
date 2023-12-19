use std::path::Path;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::transform_type::{TransformCSV, TransformXLSX};

pub(crate) fn manipulate_xlsx_data<P: AsRef<Path>>(transform_xlsx: TransformXLSX, data_path: P) -> Result<(), ParsingError> {
 let data_frames = match transform_xlsx.all_sheets {
        true => {
            parse_data::xlsx_parse::organize_structure::import_all_ordered_df(data_path, transform_xlsx.sheet_numbers,transform_xlsx.organized_bys)?
        }
        false => {
            parse_data::xlsx_parse::organize_structure::import_some_ordered_df(data_path, transform_xlsx.sheet_numbers, transform_xlsx.organized_bys)?
        }
    };
    println!("dataframes: {:?}", data_frames);
    //let ordered_bys:Vec<&OrganizedBy> = transform_xlsx.worksheets.iter().map(| sheet_info|&sheet_info.structured_by).collect();
    //let ordered_df = parse_data::xlsx_parse::organize_structure::order_data_frames(data_frames,ordered_bys);
    // manipulate data add new columns to df: https://stackoverflow.com/questions/75266448/add-column-series-to-dataframe-in-polars-rust
    Ok(())
}

pub(crate) fn manipulate_csv_data<P: AsRef<Path>>(transform_csv: TransformCSV, data_path: P) -> Result<(), ParsingError> {
    todo!()
}
