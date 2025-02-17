use super::structure::SimpleProps;
use crate::{
    error::{WriterError, WriterResult},
    model::{
        doc::RustDocument,
        field::Field,
        structure::{RustType, StructProps},
        TryFromNode, WriteNode,
    },
};
use roxmltree::Node;
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
                rust_type = RustType::Struct(StructProps::try_from_node(node, doc)?);
            }
            "simpleType" => {
                // determine simpleType's-type: enum, list
                rust_type = RustType::Simple(SimpleProps::try_from_node(node, doc)?);
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
mod tests {
    use super::*;
    use crate::model::{
        field::RustFieldType,
        structure::{Restrictions, SimpleProps},
    };

    #[test]
    fn can_read_complex_sequence() {
        const XML: &str = r#"<xs:complexType name="InstalledAppType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
        <xs:sequence minOccurs="0">
            <xs:element name="Id" type="xs:string" minOccurs="0"/>
        </xs:sequence>
    </xs:complexType>"#;

        let rust_node = node_from_xml(XML);
        assert_eq!(
            rust_node.rust_type,
            RustType::Struct(StructProps {
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
            })
        );
    }

    fn node_from_xml(xml: &str) -> RustNode {
        let doc = roxmltree::Document::parse(xml).unwrap();
        let node = doc.root_element();
        let mut rust_doc = RustDocument::default();
        let rust_node = RustNode::try_from_node(node, &mut rust_doc).expect("Failed to parse node");
        rust_node
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
        assert_eq!(
            rust_node.rust_type,
            RustType::Simple(SimpleProps {
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
                comment: Some(
                    "Represents the message keys that can be returned by response error messages".to_string()
                ),
            })
        );
    }
}
