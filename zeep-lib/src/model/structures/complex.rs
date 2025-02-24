use super::{
    Field, Namespace, Node, Rc, RustDocument, RustType, TryFromNode, WriterError, WriterResult, parse_comment,
};
use crate::model::{field::resolve_type, node::collect_namespaces_on_node};

#[derive(Debug, PartialEq, Default)]
pub struct ComplexProps {
    pub xml_name: String,
    pub fields: Vec<Field>,
    pub target_namespace: Option<Rc<Namespace>>,
    pub comment: Option<String>,
}

impl<'n> TryFromNode<'n> for ComplexProps {
    type Error = WriterError;

    // This is used for complex types
    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> Result<Self, Self::Error> {
        // check if the node has an attribute that starts with xmlns
        collect_namespaces_on_node(node, doc);

        let mut element_name = node.attribute("name");

        // check if the complexType is wrapped in an element and the name is on the element
        if element_name.is_none() {
            element_name = node.parent().and_then(|n| n.attribute("name"));
        }

        let Some(element_name) = element_name else {
            return Err(WriterError::attribute_missing(&node, "name"));
        };

        let mut result = ComplexProps {
            xml_name: element_name.to_string(),
            fields: vec![],
            target_namespace: doc.current_target_namespace.clone(),
            comment: parse_comment(node),
        };

        for n in node.children().filter(Node::is_element) {
            // check if this node is an extension of another node, if so, find the base node and copy all the fields
            // from that node.
            if n.tag_name().name() == "complexContent" {
                result = read_complex_content_node(element_name, n, doc)?;
            }

            if n.tag_name().name() == "sequence" {
                result = read_sequence_node(element_name, n, doc)?;
            }

            if n.tag_name().name() == "attribute" {
                // read it as a field and add it to the fields
                let field = Field::try_from_node(n, doc)?;
                result.fields.push(field);
            }
        }

        Ok(result)
    }
}

fn read_complex_content_node(element_name: &str, mut node: Node, doc: &mut RustDocument) -> WriterResult<ComplexProps> {
    let comment = parse_comment(node);
    let mut base_fields = vec![];
    import_extension_fields(&mut node, doc, &mut base_fields)?;

    for n in node.children().filter(Node::is_element) {
        if n.tag_name().name() == "sequence" {
            import_sequence_node_fields(&mut node, doc, &mut base_fields)?;
        }
    }

    let struct_props = ComplexProps {
        xml_name: element_name.to_string(),
        fields: base_fields,
        target_namespace: doc.current_target_namespace.clone(),
        comment,
    };

    Ok(struct_props)
}

fn read_sequence_node(element_name: &str, mut node: Node, doc: &mut RustDocument) -> WriterResult<ComplexProps> {
    let comment = parse_comment(node);
    let mut base_fields = vec![];

    import_sequence_node_fields(&mut node, doc, &mut base_fields)?;

    let struct_props = ComplexProps {
        xml_name: element_name.to_string(),
        fields: base_fields,
        target_namespace: doc.current_target_namespace.clone(),
        comment,
    };

    Ok(struct_props)
}

fn import_sequence_node_fields(
    node: &mut Node,
    doc: &mut RustDocument,
    base_fields: &mut Vec<Field>,
) -> Result<(), WriterError> {
    let children = node.children().filter(Node::is_element);

    for mut child in children {
        let tag_name = child.tag_name().name();

        if tag_name == "choice" {
            import_choice_fields(&mut child, doc, base_fields)?;
            continue;
        }

        if tag_name == "sequence" {
            // nested sequence
            return import_sequence_node_fields(&mut child, doc, base_fields);
        }

        // regular field
        let field = Field::try_from_node(child, doc)?;

        // add the fields to the base fields to maintain the order
        base_fields.push(field);
    }

    Ok(())
}

fn import_choice_fields(
    node: &mut Node,
    doc: &mut RustDocument,
    base_fields: &mut Vec<Field>,
) -> Result<(), WriterError> {
    import_sequence_node_fields(node, doc, base_fields)
}

fn import_extension_fields(node: &mut Node, doc: &mut RustDocument, base_fields: &mut Vec<Field>) -> WriterResult<()> {
    if let Some(base) = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "extension")
    {
        // look in doc for the node with the same xml_name
        let xml_name = base
            .attribute("base")
            .ok_or_else(|| WriterError::attribute_missing(node, "base"))?;
        let (xml_name, namespace_abbreviation) = resolve_type(xml_name, doc);
        let base_node = doc
            .find_node_by_xml_name(node, xml_name, namespace_abbreviation.as_deref())
            .ok_or_else(|| WriterError::NodeNotFound(xml_name.to_string()))?;

        match &base_node.rust_type {
            RustType::Ignore => {
                // unsupported type
            }
            RustType::Complex(struct_props) => {
                base_fields.clone_from(&struct_props.fields);
            }
            RustType::Element(_element_props) => {
                // unsupported type
            }
            RustType::Simple(_simple_props) => {
                // unsupported type
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{node::test_utils::parse_from_xml, structures::element::ElementProps};

    #[test]
    fn can_read_nested_sequence_of_any_type() {
        const XML: &str = r#"
<xs:complexType name="SearchDiagnosticsType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
    <xs:sequence>
      <xs:sequence>
        <xs:any processContents="skip" minOccurs="0" maxOccurs="unbounded" namespace="\#\#any"/>
      </xs:sequence>
    </xs:sequence>
 </xs:complexType>
 "#;

        let doc = roxmltree::Document::parse(XML).unwrap();
        let rust_node = parse_from_xml::<ComplexProps>(&doc);
        assert_eq!(rust_node.xml_name, "SearchDiagnosticsType");
    }

    #[test]
    fn can_parse_sequence_and_attribute() {
        const XML: &str = r#"
<xs:complexType name="ReplyBody" xmlns:xs="http://www.w3.org/2001/XMLSchema">
    <xs:sequence>
      <xs:element minOccurs="0" maxOccurs="1" name="Message" type="xs:string" />
    </xs:sequence>
    <xs:attribute ref="xml:lang" use="optional" />
</xs:complexType>
"#;

        let doc = roxmltree::Document::parse(XML).unwrap();
        let rust_node = parse_from_xml::<ComplexProps>(&doc);
        assert_eq!(rust_node.xml_name, "ReplyBody");
    }
}
