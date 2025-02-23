use super::{Node, RustDocument, RustFieldType, TryFromNode, WriterError, complex::ComplexProps};
use crate::model::{
    field::{OtherRustType, as_rust_type},
    node::collect_namespaces_on_node,
};

#[derive(Debug, PartialEq)]
pub struct ElementProps {
    pub xml_name: String,
    pub element_type: ElementType,
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    RustType(RustFieldType),
    ComplexType(ComplexProps),
    Unsupported,
}

impl ElementProps {
    pub fn rust_type(&self) -> Option<RustFieldType> {
        match &self.element_type {
            ElementType::RustType(rust_type) => Some(rust_type.clone()),
            ElementType::ComplexType(complex_props) => Some(RustFieldType::Other(OtherRustType {
                name: complex_props.xml_name.clone(),
                module: complex_props
                    .target_namespace
                    .as_ref()
                    .map(|ns| ns.rust_mod_name.clone()),
            })),
            ElementType::Unsupported => None,
        }
    }
}

impl<'n> TryFromNode<'n> for ElementProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        // check if the node has an attribute that starts with xmlns
        collect_namespaces_on_node(node, doc);

        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::attribute_missing(&node, "name"))?
            .to_string();

        if let Some(rust_type) = node.attribute("type").map(|t| as_rust_type(t, doc)) {
            return Ok(ElementProps {
                xml_name,
                element_type: ElementType::RustType(rust_type),
            });
        }

        // check for complexType
        for n in node.children().filter(Node::is_element) {
            if n.tag_name().name() == "complexType" {
                let complex_props = ComplexProps::try_from_node(n, doc)?;
                return Ok(ElementProps {
                    xml_name,
                    element_type: ElementType::ComplexType(complex_props),
                });
            }
        }

        Ok(ElementProps {
            xml_name,
            element_type: ElementType::Unsupported,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        field::{OtherRustType, RustFieldType},
        node::test_utils::parse_from_xml,
    };

    #[test]
    fn can_read_element() {
        const XML: &str = r#"<xs:element name="Path" abstract="true" type="t:BasePathToElementType" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types" />"#;
        let doc = roxmltree::Document::parse(XML).unwrap();
        let rust_node = parse_from_xml::<ElementProps>(&doc);
        assert_eq!(rust_node.xml_name, "Path");

        let ElementType::RustType(rust_type) = rust_node.element_type else {
            panic!("Expected RustType, got {:?}", rust_node.element_type);
        };

        assert_eq!(
            rust_type,
            RustFieldType::Other(OtherRustType {
                name: "BasePathToElementType".to_string(),
                module: Some("mod_typ".to_string()),
            })
        );
    }

    #[test]
    fn can_read_element_with_complex_content() {
        const XML: &str = r#"<s:element name="GetNoCustContactsByTime" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap/" xmlns:mime="http://schemas.xmlsoap.org/wsdl/mime/" xmlns:SOAP-ENC="http://schemas.xmlsoap.org/soap/encoding/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:s0="http://ws.db.ccmm.applications.nortel.com" xmlns:s="http://www.w3.org/2001/XMLSchema" xmlns="http://schemas.xmlsoap.org/wsdl/" targetNamespace="http://ws.db.ccmm.applications.nortel.com" xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/">
        <s:complexType>
          <s:sequence>
            <s:element minOccurs="0" name="customerID" type="s:long" />
            <s:element minOccurs="0" name="startTime" type="s:dateTime" />
            <s:element minOccurs="0" name="sessionKey" type="s:string" />
          </s:sequence>
        </s:complexType>
      </s:element>"#;

        let doc = roxmltree::Document::parse(XML).unwrap();
        let rust_node = parse_from_xml::<ElementProps>(&doc);
        assert_eq!(rust_node.xml_name, "GetNoCustContactsByTime");
    }
}
