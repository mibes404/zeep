pub mod doc;
pub mod field;
mod file_header;
mod helpers;
mod helpers_content;
pub mod node;
pub mod soap;
pub mod structures;

use crate::model::doc::RustDocument;
use roxmltree::Node;

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
