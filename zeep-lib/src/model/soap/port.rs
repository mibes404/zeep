use super::message::SoapMessage;
use crate::{
    error::{WriterError, WriterResult},
    model::{TryFromNode, field::resolve_type},
};
use std::{collections::HashMap, rc::Rc};

type XmlName = String;

pub struct SoapPort {
    pub xml_name: String,
    pub operations: HashMap<XmlName, SoapOperation>,
}

pub struct SoapOperation {
    pub input: SoapOperationMapping,
    pub output: Option<SoapOperationMapping>,
}

pub struct SoapOperationMapping {
    pub message: Rc<SoapMessage>,
}

impl<'n> TryFromNode<'n> for SoapPort {
    type Error = WriterError;

    fn try_from_node(node: roxmltree::Node<'n, 'n>, doc: &mut crate::model::doc::RustDocument) -> WriterResult<Self> {
        let xml_name = node
            .attribute("name")
            .ok_or_else(|| WriterError::attribute_missing(&node, "name"))?
            .to_string();

        let operations = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "operation")
            .map(|o| {
                let name = o
                    .attribute("name")
                    .ok_or_else(|| WriterError::attribute_missing(&o, "name"))?
                    .to_string();
                let opp = SoapOperation::try_from_node(o, doc)?;
                Ok((name, opp))
            })
            .collect::<WriterResult<HashMap<XmlName, SoapOperation>>>()?;

        Ok(SoapPort { xml_name, operations })
    }
}

impl<'n> TryFromNode<'n> for SoapOperation {
    type Error = WriterError;

    fn try_from_node(node: roxmltree::Node<'n, 'n>, doc: &mut crate::model::doc::RustDocument) -> WriterResult<Self> {
        let input = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "input")
            .map_or(Err(WriterError::NodeNotFound("input".to_string())), |n| {
                read_port_operation(doc, n)
            })?;

        let output = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "output")
            .and_then(|n| read_port_operation(doc, n).ok());

        Ok(SoapOperation { input, output })
    }
}

fn read_port_operation(
    doc: &mut crate::model::doc::RustDocument,
    n: roxmltree::Node<'_, '_>,
) -> WriterResult<SoapOperationMapping> {
    let message_name = n
        .attribute("message")
        .ok_or_else(|| WriterError::attribute_missing(&n, "message"))?;
    let (xml_name, namespace) = resolve_type(message_name, doc);
    let soap_message = doc
        .find_message_by_xml_name(xml_name, namespace.as_deref())
        .ok_or(WriterError::MessageNotFound(xml_name.to_string()))?;
    Ok(SoapOperationMapping {
        message: soap_message.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::reader::XmlReader;

    #[test]
    fn can_read_soap_port_mappings() {
        const XML: &str = include_str!("../../../test-data/tempconverter.wsdl");
        let doc = XmlReader::read_xml_from_file("tempconverter.wsdl", XML).unwrap();
        assert_eq!(doc.soap_ports.len(), 1);
        let port = &doc.soap_ports[0];
        assert_eq!(port.operations.len(), 2);
        // check the first operation
        let first_operation = port.operations.get("CelsiusToFahrenheit").unwrap();
        assert_eq!(first_operation.input.message.xml_name, "CelsiusToFahrenheit");
        assert_eq!(
            first_operation.output.as_ref().unwrap().message.xml_name,
            "CelsiusToFahrenheitResponse"
        );
    }
}
