use crate::{
    error::{WriterError, WriterResult},
    reader::{Files, FilesToRead},
};
use std::path::Path;

/// Prepare the input file and the XSD files to be read.
///
/// # Errors
/// When the necessary files cannot be found or read.
pub fn read_input_file_and_xsd_files_at_path(current_file: &Path) -> WriterResult<FilesToRead> {
    if !current_file.is_file() {
        return Err(WriterError::PathNotFound);
    }

    let file_name = current_file.to_str().ok_or(WriterError::PathNotFound)?;

    let xml = std::fs::read_to_string(current_file)?;
    let files = Files::new(file_name, xml);

    // Imported XSDs (which can live anywhere, e.g. `../../Dictionnaires/v5.0/foo.xsd`) are
    // resolved and loaded lazily, relative to the file that imports them, while reading.
    let file_to_read = FilesToRead::new(file_name, files);
    Ok(file_to_read)
}
