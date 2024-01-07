use std::collections::HashMap;
use std::sync::Arc;
use std::fs::File;
use std::path::Path;
use arrow::array::{ArrayRef, GenericStringArray, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field};
use arrow::{
    record_batch::RecordBatch,
    datatypes::Schema,
};
use arrow::csv::{Writer, WriterBuilder};
use arrow::record_batch::RecordBatchWriter;
use parquet::{
    file::reader::SerializedFileReader,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use manipulate_data::manipulation::shape_data::ShapedData;
use crate::operations::new_path;

pub struct WrapperTable(pub Vec<ShapedData>);
// The Table struct. This object will represent the data read from the
// parquet files and it will be our entry point to any value in the file


impl WrapperTable {
    // see: https://elferherrera.github.io/arrow_guide/reading_parquet.html
    /*
    let schema = Schema::new(vec![
        Field::new("index", DataType::Int32, false),
        Field::new("fruits", DataType::Utf8, false),
    ]);

    let a = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let b = StringArray::from(vec!["apple", "banana", "pineapple", "melon", "pear"]);

    let record_batch =
        RecordBatch::try_new(Arc::new(schema), vec![Arc::new(a), Arc::new(b)]).unwrap();

     */

    pub(crate) fn to_table(&self) -> Table {
        // see: https://elferherrera.github.io/arrow_guide/arrays_recordbatch.html
        let mut record_batches: Vec<RecordBatch> = vec![];
        for data_sheet in self.0.iter() {
            // todo: something is wrong with metadata/schema
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

            let length = record_batches.len();
            Table {
                schema: Schema { fields: Default::default(), metadata: Default::default() },
                data: record_batches,
                rows: length,
            }

    }

}
pub struct Table {
    // We mantain a copy of the RecordBatch schema to keep handy the
    // file's metadata information.
    schema: Schema,
    data: Vec<RecordBatch>,
    rows: usize,
}
impl Table {
    pub fn to_parquet<T: AsRef<Path>>(&self, path: T) {
        println!("rows: {:?}", self.rows);
        println!("schema: {:?}", self.schema);
        panic!();
        let file = File::create(path).unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(self.schema.clone()), None).unwrap();

        // WriterProperties can be used to set Parquet file options
        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();
        for batch in self.data.iter() {
            writer.write(&batch).unwrap();
        }
        writer.close().unwrap();
    }
    pub(crate) fn to_csv(&self) ->  () {
        // https://docs.rs/arrow-csv/latest/arrow_csv/writer/index.html
        let ending = ".csv";
        for batch in self.data.iter() {
            let middle = &batch.schema().metadata.get("resource").unwrap().to_owned().to_owned();
            let new_path = new_path(ending, middle);
            let file = File::create(new_path).unwrap();
            // create a builder that doesn't write headers
            let builder = WriterBuilder::new().has_headers(true);
            let mut writer = builder.build(file);
            writer.write(batch).unwrap();
            writer.close().unwrap();
        }
    }

    fn new_csv_path(resource_name: &String) -> String {
        let ending = "csv";
        return new_path(ending, resource_name);
    }
}