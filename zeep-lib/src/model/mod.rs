pub mod doc;
pub mod field;
pub mod node;
pub mod soap;
pub mod structures;

mod file_header;
mod helpers;
mod helpers_content;

#[cfg(test)]
mod helpers_test;

use crate::model::doc::RustDocument;
use roxmltree::Node;

#[allow(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Namespace {
    pub namespace: String,
    pub abbreviation: String,
    pub rust_mod_name: String,
}

// Two `Namespace`s represent the same XML namespace iff their URIs match. The
// `abbreviation`/`rust_mod_name` are locally auto-generated and can legitimately
// differ between two `Namespace` instances for the same URI: each imported XSD
// file is parsed into its own throw-away `RustDocument` (see `reader::process_import`)
// with its own abbreviation-collision state, then merged into the parent document,
// so the same namespace URI can end up with different generated abbreviations
// depending on which document generated them first.
impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace
    }
}

pub trait TryFromNode<'n>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error>;
}
