use std::collections::HashMap;
use manipulate_data::manipulation::shape_data::ShapedData;
use arrow::{
    record_batch::RecordBatch,
    datatypes::Schema,
};
use parquet::{
    arrow::{ ArrowWriter},
};
use std::sync::Arc;
use std::fs::{File, metadata};
use std::path::Path;
use arrow::array::{ArrayRef, GenericByteArray, GenericStringArray, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, GenericStringType, SchemaRef};
use arrow::error::ArrowError;

pub struct WrapperTable(pub Vec<ShapedData>);
// The Table struct. This object will represent the data read from the
// parquet files and it will be our entry point to any value in the file


impl WrapperTable {
    // see: https://elferherrera.github.io/arrow_guide/reading_parquet.html
    pub(crate) fn to_table(&self) -> Table {
        // see: https://elferherrera.github.io/arrow_guide/arrays_recordbatch.html
        let mut record_batches:Vec<RecordBatch> = vec![];
        for data_sheet in self.0.iter() {
            // metadata
            let resource = &data_sheet.resource;
            let property_to_data = &data_sheet.property_to_data;
            let all_fields: Vec<Field> = property_to_data.keys().map(|property_name|Field::new(property_name, DataType::Utf8, false)).collect();
            let mut metadata:HashMap<String,String> = HashMap::new();
            metadata.insert("resource:".to_string(), resource.to_owned());
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
            record_batches.push(record_batch)
        }
        let length = record_batches.len();
        Table{
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
        let file = File::create(path).unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(self.schema.clone()), None).unwrap();

        for batch in self.data.iter() {
            writer.write(&batch).unwrap();
        }

        writer.close().unwrap();
    }
}