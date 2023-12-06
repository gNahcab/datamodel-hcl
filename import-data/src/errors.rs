use std::io;

#[derive(Debug)]
pub enum DataImportError {
    IO(io::Error),
    XlsxError(calamine::XlsxError)
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