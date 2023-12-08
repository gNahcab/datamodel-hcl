use std::io;

#[derive(Debug)]
pub enum DataImportError {
    IO(io::Error),
    XlsxError(calamine::XlsxError),
    ArrowError(arrow::error::ArrowError),
    PolarsError(polars::error::PolarsError),
    HCLError(hcl::Error)
}

impl From<calamine::XlsxError> for DataImportError {
    fn from(error: calamine::XlsxError) -> Self {
        DataImportError::XlsxError(error)
    }
}
impl From<io::Error> for DataImportError {
    fn from(error: io::Error) -> Self {
        DataImportError::IO(error)
    }
}
impl From<arrow::error::ArrowError> for DataImportError {
    fn from(error: arrow::error::ArrowError) -> Self {
        DataImportError::ArrowError(error)
    }
}
impl From<polars::error::PolarsError> for DataImportError {
    fn from(error: polars::error::PolarsError) -> Self {
        DataImportError::PolarsError(error)
    }
}
impl From<hcl::Error> for DataImportError {
    fn from(error: hcl::Error) -> Self {
        DataImportError::HCLError(error)
    }
}
