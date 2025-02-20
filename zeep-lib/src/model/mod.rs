pub mod doc;
pub mod field;
pub mod node;
pub mod structures;

use crate::{
    error::WriterResult,
    model::{doc::RustDocument, field::Field},
};
use roxmltree::Node;
use std::{
    fmt::{Display, Write},
    io,
};

#[derive(Debug, PartialEq)]
pub struct Namespace {
    pub namespace: String,
    pub abbreviation: String,
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
