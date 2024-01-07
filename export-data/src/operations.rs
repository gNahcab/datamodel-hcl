use std::error::Error;
use std::path::Path;
use std::result;
use crate::table::WrapperTable;
use chrono::{Datelike, Timelike, Utc};
use manipulate_data::manipulation::shape_data::ShapedData;

pub fn export_parquet(shaped_sheets: Vec<ShapedData>) -> ()  {
    //returns parquet-file
    let new_path = new_parquet_path();
    println!("write to new_path: {:?}", new_path);
    WrapperTable(shaped_sheets).to_table().to_parquet(new_path);
}

pub fn export_csv(shaped_sheets: Vec<ShapedData>) -> () {
    WrapperTable(shaped_sheets).to_table().to_csv();
}
fn new_parquet_path() -> String {
    let ending = ".parquet";
    let middle = "dasch_data";
    return new_path(ending, middle);
}

pub(crate) fn new_path(ending: &str, middle: &str) -> String {
    // returns a valid new path
    let now = Utc::now();
    let date = format!("{:02}{:02}{:02}",now.year(), now.month(), now.day());
    let rest = format!("{}_{}", date, middle);
    let mut path = format!("{}{}", rest, ending);
    if Path::new(path.as_str()).exists() {
        let extended = format!("{:02}_{:02}{:02}", now.hour(), now.minute(), now.second());
        path = format!("{}_{}{}", rest, extended, ending);
    }
    return path.to_string();
}