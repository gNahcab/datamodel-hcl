use std::collections::HashMap;
use std::sync::Arc;
use std::fs::File;
use std::path::Path;
use arrow::array::{ArrayRef, GenericStringArray, StringArray};
use arrow::datatypes::{DataType, Field};
use arrow::{
    record_batch::RecordBatch,
    datatypes::Schema,
};
use arrow::csv::WriterBuilder;
use arrow::record_batch::RecordBatchWriter;
use parquet::arrow::ArrowWriter;
use chrono::{Datelike, Timelike, Utc};
use manipulate_data::manipulation::shape_data::ShapedData;

pub struct WrapperExport(pub Vec<ShapedData>);
// The Table struct. This object will represent the data read from the
// parquet files and it will be our entry point to any value in the file

impl WrapperExport {
    // see: https://elferherrera.github.io/arrow_guide/reading_parquet.html
    pub(crate) fn to_dataholder(&self) -> DataHolder {
        // see: https://elferherrera.github.io/arrow_guide/arrays_recordbatch.html
        let mut record_batches: Vec<RecordBatch> = vec![];
        for data_sheet in self.0.iter() {
            // metadata
            let resource = &data_sheet.resource;
            let property_to_data = &data_sheet.property_to_data;
            let all_fields: Vec<Field> = property_to_data.keys().map(|property_name|Field::new(property_name, DataType::Utf8, false)).collect();
            let mut metadata:HashMap<String,String> = HashMap::new();
            metadata.insert("resource".to_string(), resource.to_owned());
            let schema: Schema = Schema::new_with_metadata(all_fields, metadata);
            // data
            let data: Vec<Vec<String>> = property_to_data.values().map(|value|value.to_owned()).collect();
            let array: Vec<GenericStringArray<i32>> = data.iter().map(|vector|StringArray::from(vector.to_owned())).collect();
            let arcs: Vec<Arc<dyn arrow::array::Array>> = array.iter().map(|array|Arc::new(array.to_owned()) as ArrayRef).collect();
            let result_record_batch = RecordBatch::try_new(Arc::new(schema),arcs);
            let record_batch = match result_record_batch {
                Ok(value) => {value}
                Err(err) => {
                    println!("{:?}", err);
                    panic!();
                }
            };
            record_batches.push(record_batch);
        }
            DataHolder {
                data: record_batches,
            }

    }

}
pub struct DataHolder {
    data: Vec<RecordBatch>,
}
impl DataHolder {
    pub fn to_parquet(&self) {
        let ending = ".parquet";
        for batch in self.data.iter() {
            let middle = &batch.schema().metadata.get("resource").unwrap().to_owned().to_owned();
            let new_path = new_path(ending, middle);
            let file = File::create(&new_path).unwrap();
            let mut writer = ArrowWriter::try_new(file, batch.schema(), None).unwrap();
            writer.write(&batch).unwrap();
            writer.close().unwrap();
            println!("wrote new parquet-file: {}", new_path);
        }
    }
    pub(crate) fn to_csv(&self) ->  () {
        // https://docs.rs/arrow-csv/latest/arrow_csv/writer/index.html
        let ending = ".csv";
        for batch in self.data.iter() {
            let middle = &batch.schema().metadata.get("resource").unwrap().to_owned().to_owned();
            let new_path = new_path(ending, middle);
            let file = File::create(&new_path).unwrap();

            // create a builder that doesn't write headers
            let builder = WriterBuilder::new().has_headers(true);
            let mut writer = builder.build(file);
            writer.write(batch).unwrap();
            writer.close().unwrap();
            println!("wrote new csv-file: {}", new_path);
        }
    }
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
