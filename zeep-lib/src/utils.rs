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

    let file_name = current_file
        .file_name()
        .ok_or(WriterError::PathNotFound)?
        .to_str()
        .ok_or(WriterError::PathNotFound)?;

    let xml = std::fs::read_to_string(current_file)?;
    let mut files = Files::new(file_name, xml);

    for entry in current_file.parent().ok_or(WriterError::PathNotFound)?.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "xsd" && !current_file.eq(&path) {
            let file_name = path
                .file_name()
                .ok_or(WriterError::PathNotFound)?
                .to_str()
                .ok_or(WriterError::PathNotFound)?;
            let xml = std::fs::read_to_string(&path)?;
            files.add(file_name, xml);
        }
    }

    let file_to_read = FilesToRead::new(file_name, files);
    Ok(file_to_read)
}
