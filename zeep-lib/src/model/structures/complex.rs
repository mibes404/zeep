use super::*;
use crate::model::field::resolve_type;

#[derive(Debug, PartialEq)]
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
        let element_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?;

        for n in node.children().filter(Node::is_element) {
            // check if this node is an extension of another node, if so, find the base node and copy all the fields
            // from that node.
            if n.tag_name().name() == "complexContent" {
                return read_complex_content_node(element_name, n, doc);
            }

            if n.tag_name().name() == "sequence" {
                return read_sequence_node(element_name, n, doc);
            }
        }

        // empty struct
        Ok(ComplexProps {
            xml_name: element_name.to_string(),
            fields: vec![],
            target_namespace: doc.current_target_namespace.clone(),
            comment: parse_comment(node),
        })
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
    let fields = node
        .children()
        .filter(Node::is_element)
        .map(|n| Field::try_from_node(n, doc))
        .collect::<WriterResult<Vec<Field>>>()?;

    // add the fields to the base fields to maintain the order
    base_fields.extend(fields);
    Ok(())
}

fn import_extension_fields(node: &mut Node, doc: &mut RustDocument, base_fields: &mut Vec<Field>) -> WriterResult<()> {
    if let Some(base) = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "extension")
    {
        // look in doc for the node with the same xml_name
        let xml_name = base
            .attribute("base")
            .ok_or_else(|| WriterError::AttributeMissing("base".to_string()))?;
        let (xml_name, namespace_abbreviation) = resolve_type(xml_name, doc);
        let base_node = doc
            .find_node_by_xml_name(xml_name, namespace_abbreviation.as_deref())
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
