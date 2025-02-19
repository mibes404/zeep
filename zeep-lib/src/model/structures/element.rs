use super::*;

#[derive(Debug, PartialEq)]
pub struct ElementProps {
    pub xml_name: String,
    pub rust_type: RustFieldType,
}

impl<'n> TryFromNode<'n> for ElementProps {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, _doc: &mut RustDocument) -> Result<Self, Self::Error> {
        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        let rust_type = node
            .attribute("type")
            .map(RustFieldType::from)
            .ok_or_else(|| WriterError::AttributeMissing("type".to_string()))?;

        Ok(ElementProps { xml_name, rust_type })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{field::RustFieldType, node::test_utils::parse_from_xml};

    #[test]
    fn can_read_element() {
        const XML: &str = r#"<xs:element name="Path" abstract="true" type="t:BasePathToElementType" xmlns:xs="http://www.w3.org/2001/XMLSchema" />"#;
        let doc = roxmltree::Document::parse(XML).unwrap();
        let rust_node = parse_from_xml::<ElementProps>(&doc);
        let expected = ElementProps {
            xml_name: "Path".to_string(),
            rust_type: RustFieldType::Other("BasePathToElementType".to_string()),
        };
        assert_eq!(rust_node, expected);
    }
}
