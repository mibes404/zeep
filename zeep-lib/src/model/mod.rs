pub mod doc;
pub mod field;
pub mod node;
pub mod structures;

use crate::{
    error::WriterResult,
    model::doc::RustDocument,
};
use roxmltree::Node;
use std::io;

#[derive(Debug, PartialEq)]
pub struct Namespace {
    pub namespace: String,
    pub abbreviation: String,
    pub rust_mod_name: String,
}

pub trait TryFromNode<'n>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error>;
}

pub trait WriteXml<W>
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()>;
}
