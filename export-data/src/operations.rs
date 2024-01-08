use crate::export::WrapperExport;
use manipulate_data::manipulation::shape_data::ShapedData;

pub fn export_parquet(shaped_sheets: Vec<ShapedData>) -> ()  {
    //returns parquet-files
    WrapperExport(shaped_sheets).to_dataholder().to_parquet();
}

pub fn export_csv(shaped_sheets: Vec<ShapedData>) -> () {
    //returns csv-files
    WrapperExport(shaped_sheets).to_dataholder().to_csv();
}
