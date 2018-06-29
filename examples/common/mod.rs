use std::path::Path;

use agnes::source::{CsvSource, CsvReader};
use agnes::error::*;

pub fn load_csv(filename: &str) -> Result<CsvReader> {
    let data_filepath = Path::new(file!()) // start as this file
        .parent().unwrap()                 // navigate up to common module directory
        .parent().unwrap()                 // navigate up to examples directory
        .parent().unwrap()                 // navigate up to root directory
        .join("data")                      // navigate into data directory
        .join(filename);                   // navigate to target file

    let source = CsvSource::new(data_filepath.into())?;
    Ok(CsvReader::new(&source)?)
}
