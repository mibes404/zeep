use super::structures::{complex::ComplexProps, element::ElementProps, simple::SimpleProps, RustType};
use crate::{
    error::{WriterError, WriterResult},
    model::{doc::RustDocument, field::Field, TryFromNode, WriteNode},
};
use roxmltree::{Document, Node};
use std::io;

pub struct RustNode {
    pub rust_type: RustType,
}

impl<'n> TryFromNode<'n> for RustNode {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> WriterResult<Self> {
        if !node.is_element() {
            return Err(WriterError::NotAnElement);
        }

        if let Some(target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(target_namespace);
        }

        let mut rust_type = RustType::Ignore;
        match node.tag_name().name() {
            "complexType" => {
                // determine complexType's-type: struct, enum, list
                rust_type = RustType::Complex(ComplexProps::try_from_node(node, doc)?.into());
            }
            "simpleType" => {
                // determine simpleType's-type: enum, list
                rust_type = RustType::Simple(SimpleProps::try_from_node(node, doc)?.into());
            }
            "element" => {
                // determine element's-type: struct, enum, list
                rust_type = RustType::Element(ElementProps::try_from_node(node, doc)?.into());
            }
            _ => {}
        }

        Ok(RustNode { rust_type })
    }
}

impl<W> WriteNode<W> for RustNode
where
    W: io::Write,
{
    fn write_node(&self, writer: &mut W) -> WriterResult<()> {
        if self.rust_type == RustType::Ignore {
            return Ok(());
        }

        self.rust_type.write_node(writer)
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub fn parse_from_xml<'n, N>(doc: &'n Document<'n>) -> N
    where
        N: TryFromNode<'n>,
        N::Error: std::fmt::Debug,
    {
        let node = doc.root_element();
        let mut rust_doc = RustDocument::default();
        let rust_node = N::try_from_node(node, &mut rust_doc).expect("Failed to parse node");
        rust_node
    }
}

#[cfg(test)]
mod tests {
    use super::{test_utils::parse_from_xml, *};
    use crate::model::{field::RustFieldType, structures::Restrictions};

    #[test]
    fn can_read_complex_sequence() {
        const XML: &str = r#"<xs:complexType name="InstalledAppType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
        <xs:sequence minOccurs="0">
            <xs:element name="Id" type="xs:string" minOccurs="0"/>
        </xs:sequence>
    </xs:complexType>"#;

        let rust_node = node_from_xml(XML);
        let expected = ComplexProps {
            xml_name: "InstalledAppType".to_string(),
            fields: vec![Field {
                xml_name: "Id".to_string(),
                rust_name: "id".to_string(),
                rust_type: RustFieldType::String,
                optional: true,
                vec: false,
                target_namespace: None,
            }],
            target_namespace: None,
            comment: None,
        }
        .into();

        assert_eq!(rust_node.rust_type, RustType::Complex(expected));
    }

    fn node_from_xml(xml: &str) -> RustNode {
        let doc = Document::parse(xml).unwrap();
        parse_from_xml(&doc)
    }

    #[test]
    fn can_read_simple_enum() {
        const XML: &str = r#"<xs:simpleType name="ResponseCodeType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
        <xs:annotation>
            <xs:documentation>
                Represents the message keys that can be returned by response error messages
            </xs:documentation>
        </xs:annotation>
        <xs:restriction base="xs:string">
            <xs:enumeration value="NoError"/>
            <xs:enumeration value="ErrorAccessDenied"/>
            <xs:enumeration value="ErrorAccessModeSpecified"/>           
        </xs:restriction>
    </xs:simpleType>"#;
        let rust_node = node_from_xml(XML);
        let expected = SimpleProps {
            xml_name: "ResponseCodeType".to_string(),
            rust_type: RustFieldType::String,
            target_namespace: None,
            restrictions: Some(Restrictions {
                enumeration: Some(vec![
                    "NoError".to_string(),
                    "ErrorAccessDenied".to_string(),
                    "ErrorAccessModeSpecified".to_string(),
                ]),
                ..Restrictions::default()
            }),
            comment: Some("Represents the message keys that can be returned by response error messages".to_string()),
        }
        .into();

        assert_eq!(rust_node.rust_type, RustType::Simple(expected));
    }
}
