use std::{error, fmt};

pub type WriterResult<T> = Result<T, WriterError>;

#[derive(Debug, Clone)]
pub struct WriterError {
    pub message: String,
}

impl fmt::Display for WriterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "writer error: {}", self.message)
    }
}

impl error::Error for WriterError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for WriterError {
    fn from(err: std::io::Error) -> Self {
        WriterError {
            message: err.to_string(),
        }
    }
}

impl From<roxmltree::Error> for WriterError {
    fn from(err: roxmltree::Error) -> Self {
        WriterError {
            message: err.to_string(),
        }
    }
}
