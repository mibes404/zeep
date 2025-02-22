use super::{Node, RustDocument, RustFieldType, TryFromNode, WriterError};
use crate::model::{field::as_rust_type, node::collect_namespaces_on_node};

#[derive(Debug, PartialEq)]
pub struct ElementProps {
    pub xml_name: String,
    pub rust_type: RustFieldType,
}

impl<'n> TryFromNode<'n> for ElementProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        // check if the node has an attribute that starts with xmlns
        collect_namespaces_on_node(node, doc);

        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        let rust_type = node
            .attribute("type")
            .map_or(RustFieldType::String, |t| as_rust_type(t, doc));

        Ok(ElementProps { xml_name, rust_type })
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
        let expected = ElementProps {
            xml_name: "Path".to_string(),
            rust_type: RustFieldType::Other(OtherRustType {
                name: "BasePathToElementType".to_string(),
                module: Some("mod_typ".to_string()),
            }),
        };
        assert_eq!(rust_node, expected);
    }
}
