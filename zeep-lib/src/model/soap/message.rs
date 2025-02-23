use crate::{
    error::{WriterError, WriterResult},
    model::{Namespace, TryFromNode, field::resolve_type, node::RustNode},
};
use roxmltree::Node;
use std::{collections::HashMap, rc::Rc};

type XmlName = String;

/// `SoapMessage` is a struct that represents a soap message
/// These are just used for lookup and are not written to the output
pub struct SoapMessage {
    pub xml_name: String,
    pub parts: HashMap<XmlName, (Rc<RustNode>, Option<Rc<Namespace>>)>,
}

impl<'n> TryFromNode<'n> for SoapMessage {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut crate::model::doc::RustDocument) -> WriterResult<Self> {
        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::attribute_missing(&node, "name"))?
            .to_string();

        let parts = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "part")
            .map(|n| {
                let part_name = n
                    .attribute("name")
                    .ok_or_else(|| WriterError::attribute_missing(&n, "name"))?
                    .to_string();

                let element = n
                    .attribute("element")
                    .ok_or_else(|| WriterError::attribute_missing(&n, "element"))?;

                let (xml_name, namespace) = resolve_type(element, doc);
                let rust_node = doc
                    .find_node_by_xml_name(&node, xml_name, namespace.as_deref())
                    .ok_or(WriterError::NodeNotFound(xml_name.to_string()))?;

                Ok((part_name, (rust_node.clone(), namespace.clone())))
            })
            .collect::<WriterResult<HashMap<XmlName, (Rc<RustNode>, Option<Rc<Namespace>>)>>>()?;

        Ok(SoapMessage { xml_name, parts })
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::XmlReader;

    #[test]
    fn can_read_soap_messages() {
        const XML: &str = include_str!("../../../test-data/tempconverter.wsdl");
        let doc = XmlReader::read_xml_from_file("tempconverter.wsdl", XML).unwrap();
        assert_eq!(doc.soap_messages.len(), 4);
        let first_message = &doc.soap_messages[0];
        assert_eq!(first_message.xml_name, "CelsiusToFahrenheit");
        let (part, ns) = first_message.parts.get("CelsiusToFahrenheitRequest").unwrap();
        assert_eq!(part.rust_type.xml_name(), Some("CelsiusToFahrenheitRequest"));
        let namespace = ns.as_ref().unwrap();
        assert_eq!(namespace.rust_mod_name, "mod_tem");
    }
}
