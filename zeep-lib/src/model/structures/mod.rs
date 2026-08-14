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

    /// Returns the actual Rust identifier that this type gets written out as by
    /// [`super::WriteXml`], without any module path prefix. This is the name that
    /// other generated code (e.g. SOAP envelope structs, service client method
    /// signatures) must reference this type by.
    pub fn rust_type_name(&self) -> Option<String> {
        match self {
            RustType::Ignore => None,
            RustType::Complex(props) => Some(xml_name_to_rust_name(&props.xml_name)),
            RustType::Simple(props) => Some(xml_name_to_rust_name(&props.xml_name)),
            RustType::Element(props) => Some(match &props.element_type {
                // an element declared with `type="..."` is written out as a plain
                // type alias to whatever that referenced type resolves to
                ElementType::RustType(rust_type) => rust_type.plain_name(),
                // an element with an inline complexType gets its own struct, named
                // after the element itself
                ElementType::ComplexType(complex_props) => xml_name_to_rust_name(&complex_props.xml_name),
                ElementType::Unsupported => xml_name_to_rust_name(&props.xml_name),
            }),
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

#[cfg(test)]
mod rust_type_name_tests {
    use super::*;
    use crate::model::field::OtherRustType;

    #[test]
    fn element_with_referenced_type_uses_the_referenced_type_name() {
        // e.g. <xs:element name="entete" type="tec:EnteteType"/>
        let rust_type = RustType::Element(Box::new(ElementProps {
            xml_name: "entete".to_string(),
            element_type: ElementType::RustType(RustFieldType::Other(OtherRustType::new(
                "EnteteType".to_string(),
                Some("mod_tec".to_string()),
            ))),
        }));

        assert_eq!(rust_type.rust_type_name(), Some("EnteteType".to_string()));
    }

    #[test]
    fn element_with_inline_complex_type_uses_pascal_cased_element_name() {
        // e.g. <xs:element name="rechercherPoint"><xs:complexType>...</xs:complexType></xs:element>
        let rust_type = RustType::Element(Box::new(ElementProps {
            xml_name: "rechercherPoint".to_string(),
            element_type: ElementType::ComplexType(ComplexProps {
                xml_name: "rechercherPoint".to_string(),
                ..Default::default()
            }),
        }));

        assert_eq!(rust_type.rust_type_name(), Some("RechercherPoint".to_string()));
    }

    #[test]
    fn complex_type_uses_pascal_cased_xml_name() {
        let rust_type = RustType::Complex(Box::new(ComplexProps {
            xml_name: "AcquittementType".to_string(),
            ..Default::default()
        }));

        assert_eq!(rust_type.rust_type_name(), Some("AcquittementType".to_string()));
    }
}
