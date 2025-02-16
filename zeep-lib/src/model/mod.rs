pub mod doc;
pub mod field;
pub mod node;
pub mod structure;

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
pub struct TargetNamespace {
    pub namespace: String,
    pub abbreviation: String,
}

pub trait TryFromNode<'n>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error>;
}

pub trait WriteNode<W>
where
    W: io::Write,
{
    fn write_node(&self, writer: &mut W) -> WriterResult<()>;
}

fn split_type(node_type: &str) -> &str {
    node_type.split(':').next_back().unwrap_or("String")
}
