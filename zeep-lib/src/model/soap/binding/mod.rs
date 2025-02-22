pub mod writer;

use super::port::{self, SoapPort};
use crate::{
    error::{WriterError, WriterResult},
    model::{Namespace, TryFromNode, doc::RustDocument, field::resolve_type, node::RustNode},
};
use reqwest::Url;
use roxmltree::Node;
use std::{collections::HashMap, rc::Rc};

pub type XmlName = String;
pub type SoapAction = Url;

pub struct SoapBinding {
    pub name: XmlName,
    pub operations: HashMap<XmlName, SoapOperation>,
    pub target_namespaces: Vec<Rc<Namespace>>,
}

pub struct SoapOperation {
    pub action: Option<SoapAction>,
    pub input: SoapEnvelope,
    pub output: Option<SoapEnvelope>,
}

pub struct SoapEnvelope {
    pub headers: Vec<(XmlName, Rc<RustNode>)>,
    pub body: Rc<RustNode>,
}

#[derive(PartialEq, Clone, Copy)]
enum InputOrOutput {
    Input,
    Output,
}

impl<'n> TryFromNode<'n> for SoapBinding {
    type Error = WriterError;

    fn try_from_node(node: Node<'n, 'n>, doc: &mut RustDocument) -> WriterResult<Self> {
        let name = node
            .attribute("name")
            .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
            .to_string();

        let port_type = node
            .attribute("type")
            .ok_or_else(|| WriterError::AttributeMissing("type".to_string()))?;
        let (port_type_name, namespace) = resolve_type(port_type, doc);
        let port_type_node = doc
            .find_port_by_xml_name(port_type_name, namespace.as_deref())
            .ok_or(WriterError::NodeNotFound(port_type_name.to_string()))?
            .clone();

        let operations = node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "operation")
            .map(|o| {
                let operation_name = o
                    .attribute("name")
                    .ok_or_else(|| WriterError::AttributeMissing("name".to_string()))?
                    .to_string();

                let opp = read_soap_operation(o, doc, &port_type_node, &operation_name)?;
                Ok((operation_name, opp))
            })
            .collect::<WriterResult<HashMap<XmlName, SoapOperation>>>()?;

        let target_namespaces = doc.target_namespaces.clone();
        Ok(SoapBinding {
            name,
            operations,
            target_namespaces,
        })
    }
}

fn read_soap_operation<'n>(
    node: Node<'n, 'n>,
    doc: &mut RustDocument,
    port_type_node: &SoapPort,
    operation_name: &str,
) -> WriterResult<SoapOperation> {
    let port_operation = port_type_node
        .operations
        .get(operation_name)
        .ok_or(WriterError::NodeNotFound(operation_name.to_string()))?;

    // see if there is an embedded soap action
    let mut soap_actions = node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "operation")
        .filter_map(|o| o.attribute("soapAction"))
        .filter(|a| !a.is_empty())
        .map(|a| a.parse().map_err(|_| WriterError::InvalidUrl))
        .collect::<WriterResult<Vec<SoapAction>>>()?;
    let action = soap_actions.pop();

    // find the input and output nodes
    let input = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "input")
        .map_or(Err(WriterError::NodeNotFound("input".to_string())), |n| {
            read_port_operation(doc, n, port_operation, InputOrOutput::Input)
        })?;

    let output = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "output")
        .and_then(|n| read_port_operation(doc, n, port_operation, InputOrOutput::Output).ok());

    Ok(SoapOperation { action, input, output })
}

fn read_port_operation<'n>(
    doc: &mut RustDocument,
    n: Node<'n, 'n>,
    port_operation: &port::SoapOperation,
    in_or_out: InputOrOutput,
) -> WriterResult<SoapEnvelope> {
    let operation_name = n.attribute("name");

    // lookup the body and header messages on the port type
    let body = n
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "body")
        .map_or(Err(WriterError::NodeNotFound("body".to_string())), |n| {
            read_body_port_message(doc, n, port_operation, in_or_out, operation_name)
        })?;

    let headers = n
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "header")
        .map(|n| read_header_port_message(doc, n, port_operation, in_or_out, operation_name))
        .collect::<WriterResult<Vec<(XmlName, Rc<RustNode>)>>>()?;

    Ok(SoapEnvelope { headers, body })
}

fn read_body_port_message<'n>(
    doc: &mut RustDocument,
    node: Node<'n, 'n>,
    port_operation: &port::SoapOperation,
    in_or_out: InputOrOutput,
    operation_name: Option<&str>,
) -> WriterResult<Rc<RustNode>> {
    // for now we only support literal encoding
    let encoding = node
        .attribute("use")
        .ok_or_else(|| WriterError::AttributeMissing("use".to_string()))?;

    if encoding != "literal" {
        return Err(WriterError::UnsupportedEncoding(encoding.to_string()));
    }

    let parts = node.attribute("parts");

    if let Some(parts) = parts {
        // lookup the message on the port type
        let rust_node = map_to_rust_node(doc, port_operation, in_or_out, parts)?;
        return Ok(rust_node);
    }

    let Some(operation_name) = operation_name else {
        return Err(WriterError::NodeNotFound("operation_name".to_string()));
    };

    // if there are no parts defined we assume that the message is the same as the operation name
    let (_name, (rust_node, namespace)) = match in_or_out {
        InputOrOutput::Input => port_operation
            .input
            .message
            .parts
            .iter()
            .next()
            .ok_or(WriterError::NodeNotFound(operation_name.to_string()))?,
        InputOrOutput::Output => port_operation
            .output
            .as_ref()
            .and_then(|o| o.message.parts.iter().next())
            .ok_or(WriterError::NodeNotFound(operation_name.to_string()))?,
    };

    Ok(rust_node.clone())
}

fn map_to_rust_node(
    doc: &mut RustDocument,
    port_operation: &port::SoapOperation,
    in_or_out: InputOrOutput,
    parts: &str,
) -> WriterResult<Rc<RustNode>> {
    let (xml_name, _namespace) = resolve_type(parts, doc);
    let (rust_node, namespace) = if in_or_out == InputOrOutput::Input {
        port_operation
            .input
            .message
            .parts
            .get(xml_name)
            .ok_or(WriterError::NodeNotFound(xml_name.to_string()))?
    } else {
        port_operation
            .output
            .as_ref()
            .and_then(|o| o.message.parts.get(xml_name))
            .ok_or(WriterError::NodeNotFound(xml_name.to_string()))?
    };
    Ok(rust_node.clone())
}

fn read_header_port_message<'n>(
    doc: &mut RustDocument,
    n: Node<'n, 'n>,
    port_operation: &port::SoapOperation,
    in_or_out: InputOrOutput,
    _operation_name: Option<&str>,
) -> WriterResult<(XmlName, Rc<RustNode>)> {
    // get header part
    let part = n
        .attribute("part")
        .ok_or_else(|| WriterError::AttributeMissing("part".to_string()))?;

    // lookup the message on the port type
    let rust_node = map_to_rust_node(doc, port_operation, in_or_out, part)?;
    Ok((part.to_string(), rust_node))
}

#[cfg(test)]
mod tests {
    use crate::{reader::XmlReader, utils::read_input_file_and_xsd_files_at_path};
    use std::path::Path;

    #[test]
    fn can_read_soap_binding() {
        const XML: &str = include_str!("../../../../test-data/tempconverter.wsdl");
        let doc = XmlReader::read_xml_from_file("tempconverter.wsdl", XML).unwrap();
        assert_eq!(doc.soap_bindings.len(), 1);
        let binding = &doc.soap_bindings[0];
        assert_eq!(binding.name, "TempConverterEndpointServiceSoapBinding");
        assert_eq!(binding.operations.len(), 2);
        let first_operation = binding.operations.get("CelsiusToFahrenheit").unwrap();
        assert_eq!(
            first_operation.input.body.rust_type.xml_name(),
            Some("CelsiusToFahrenheitRequest")
        );
        assert_eq!(
            first_operation.output.as_ref().unwrap().body.rust_type.xml_name(),
            Some("CelsiusToFahrenheitResponse")
        );
    }

    #[test]
    fn can_read_soap_binding_with_multiple_parts() {
        let wsdl_path = Path::new("./test-data/exchange/services.wsdl");
        let files = read_input_file_and_xsd_files_at_path(wsdl_path).expect("can not read input file");
        let document = XmlReader::read_xml(&files).expect("can not read xml");
        assert_eq!(document.soap_bindings.len(), 1);
        let binding = &document.soap_bindings[0];
        assert_eq!(binding.name, "ExchangeServiceBinding");
        assert_eq!(binding.operations.len(), 122);
    }
}
