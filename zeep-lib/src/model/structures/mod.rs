pub mod complex;
pub mod element;
pub mod restrictions;
pub mod simple;
pub mod writer;

use super::{TryFromNode, field::RustFieldType};
use crate::{
    error::{WriterError, WriterResult},
    model::{Namespace, doc::RustDocument, field::Field},
    reader::WriteXml,
};
use complex::ComplexProps;
use element::{ElementProps, ElementType};
use inflector::cases::pascalcase::to_pascal_case;
use roxmltree::Node;
use simple::SimpleProps;
use std::{io, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum RustType {
    Ignore,
    Complex(Box<ComplexProps>),
    Simple(Box<SimpleProps>),
    Element(Box<ElementProps>),
}

impl RustType {
    pub fn xml_name(&self) -> Option<&str> {
        match self {
            RustType::Complex(props) => Some(&props.xml_name),
            RustType::Simple(props) => Some(&props.xml_name),
            RustType::Element(props) => Some(&props.xml_name),
            RustType::Ignore => None,
        }
    }

    pub fn try_as_element(&self) -> Option<&ElementProps> {
        match self {
            RustType::Element(props) => Some(&**props),
            _ => None,
        }
    }
}

/// check for documentation
fn parse_comment<'n>(node: Node<'n, 'n>) -> Option<String> {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == "annotation")
        .and_then(|n| {
            n.children()
                .find(|n| n.is_element() && n.tag_name().name() == "documentation")
                .and_then(|n| n.text())
        })
        .map(|s| {
            // strip all whitespace and newlines from start and end
            s.trim().to_string()
        })
}

pub fn xml_name_to_rust_name(xml_name: &str) -> String {
    to_pascal_case(xml_name)
}
