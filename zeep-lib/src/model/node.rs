use crate::{
    error::{WriterError, WriterResult},
    model::{
        doc::RustDocument,
        field::Field,
        rust_type::{RustType, StructProps},
        TargetNamespace, TryFromNode, WriteNode,
    },
};
use roxmltree::Node;
use std::{io, rc::Rc};

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
        if node.tag_name().name() == "complexType" {
            // determine complexType's-type: struct, enum, list
            rust_type = read_complex_type_node(node, doc)?;
        }

        Ok(RustNode { rust_type })
    }
}

fn read_complex_type_node(node: Node, doc: &mut RustDocument) -> WriterResult<RustType> {
    let element_name = node
        .attribute("name")
        .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?;

    for n in node.children().filter(Node::is_element) {
        if n.tag_name().name() == "sequence" {
            return read_sequence_node(element_name, n, doc);
        }
    }

    Ok(RustType::Ignore)
}

fn read_sequence_node(element_name: &str, node: Node, doc: &mut RustDocument) -> WriterResult<RustType> {
    let fields = node
        .children()
        .filter(Node::is_element)
        .map(|n| Field::try_from_node(n, doc))
        .collect::<WriterResult<Vec<Field>>>()?;

    let struct_props = StructProps {
        xml_name: element_name.to_string(),
        fields,
        target_namespace: doc.current_target_namespace.clone(),
    };

    Ok(RustType::Struct(struct_props))
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
