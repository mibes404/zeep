use roxmltree::Node;
use std::fmt::Display;
use thiserror::Error;

pub type WriterResult<T> = Result<T, WriterError>;

#[derive(Error, Debug)]
pub enum WriterError {
    #[error("writer error: {0}")]
    Message(String),
    #[error("io error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("xml error: {source}")]
    Xml {
        #[from]
        source: roxmltree::Error,
    },
    #[error("xml error: import node does not have a namespace attribute")]
    NamespaceMissing,
    #[error("xml error: import {0} can not be resolved")]
    ImportNotFound(String),
    #[error("xml error: node is not an element")]
    NotAnElement,
    #[error("xml error: node {0} missed the attribute: {1}")]
    AttributeMissing(String, String),
    #[error("xml error: unsupported xsd type: {0}")]
    UnsupportedXsdType(String),
    #[error("xml error: node not found: {0}")]
    NodeNotFound(String),
    #[error("xml error: path not found")]
    PathNotFound,
    #[error("xml error: schema not found")]
    SchemaNotFound,
    #[error("xml error: message not found: {0}")]
    MessageNotFound(String),
    #[error("xml error: unsupported encoding: {0}")]
    UnsupportedEncoding(String),
    #[error("xml error: invalid url")]
    InvalidUrl,
}

impl WriterError {
    pub fn new<S>(message: S) -> Self
    where
        S: Display,
    {
        WriterError::Message(message.to_string())
    }

    pub fn attribute_missing<'n, S>(node: &Node<'n, 'n>, attribute: S) -> Self
    where
        S: Display,
    {
        WriterError::AttributeMissing(format!("{node:?}"), attribute.to_string())
    }
}
