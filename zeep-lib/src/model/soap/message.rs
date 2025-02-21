use crate::{
    error::{WriterError, WriterResult},
    model::{TryFromNode, field::resolve_type, node::RustNode},
};
use roxmltree::Node;
use std::{collections::HashMap, rc::Rc};

type XmlName = String;

/// `SoapMessage` is a struct that represents a soap message
/// These are just used for lookup and are not written to the output
pub struct SoapMessage {
    pub xml_name: String,
    pub parts: HashMap<XmlName, Rc<RustNode>>,
}

impl<'n> TryFromNode<'n> for SoapMessage {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut crate::model::doc::RustDocument) -> WriterResult<Self> {
        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        let parts = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "part")
            .map(|n| {
                let part_name = n
                    .attribute("name")
                    .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
                    .to_string();

                let element = n
                    .attribute("element")
                    .ok_or_else(|| WriterError::AttributeMissing("element".to_string()))?;

                let (xml_name, namespace) = resolve_type(element, doc);
                let rust_node = doc
                    .find_node_by_xml_name(xml_name, namespace.as_deref())
                    .ok_or(WriterError::NodeNotFound(xml_name.to_string()))?;

                Ok((part_name, rust_node.clone()))
            })
            .collect::<WriterResult<HashMap<XmlName, Rc<RustNode>>>>()?;

        Ok(SoapMessage { xml_name, parts })
    }
}
