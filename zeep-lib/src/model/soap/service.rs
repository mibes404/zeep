use super::binding::{SoapBinding, SoapOperation};
use crate::{
    error::{WriterError, WriterResult},
    model::{TryFromNode, field::resolve_type},
    reader::WriteXml,
};
use inflector::cases::snakecase::to_snake_case;
use reqwest::Url;
use std::{io, rc::Rc};

pub type XmlName = String;

pub struct SoapService {
    pub name: XmlName,
    pub binding: Rc<SoapBinding>,
    pub location: Url,
}

impl<'n> TryFromNode<'n> for SoapService {
    type Error = WriterError;

    fn try_from_node(node: roxmltree::Node<'n, 'n>, doc: &mut crate::model::doc::RustDocument) -> WriterResult<Self> {
        let name = node
            .attribute("name")
            .ok_or_else(|| WriterError::attribute_missing(&node, "name"))?
            .to_string();

        let port = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "port")
            .ok_or_else(|| WriterError::attribute_missing(&node, "port"))?;

        let binding = port
            .attribute("binding")
            .ok_or_else(|| WriterError::attribute_missing(&port, "binding"))?;
        let (binding_name, namespace) = resolve_type(binding, doc);
        let binding_node = doc
            .find_binding_by_xml_name(binding_name, namespace.as_deref())
            .ok_or(WriterError::NodeNotFound(binding_name.to_string()))?
            .clone();

        let location = port
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "address")
            .ok_or_else(|| WriterError::NodeNotFound("address".to_string()))?
            .attribute("location")
            .ok_or_else(|| WriterError::attribute_missing(&port, "location"))?
            .parse()
            .map_err(|_| WriterError::InvalidUrl)?;

        Ok(SoapService {
            name,
            binding: binding_node,
            location,
        })
    }
}

impl<W> WriteXml<W> for SoapService
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        // create a wrapping Rust struct for the service
        writeln!(writer, "pub struct {} {{", self.name)?;
        writeln!(writer, "    pub client: reqwest::Client,")?;
        writeln!(writer, "    pub location: String,")?;
        writeln!(writer, "    pub credentials: Option<(String, String)>,")?;
        writeln!(writer, "}}")?;

        // create an implementation for the service
        writeln!(writer, "impl {} {{", self.name)?;
        writeln!(
            writer,
            "    pub fn new(credentials: Option<(String, String)>) -> Self {{"
        )?;
        writeln!(writer, "        Self {{")?;
        writeln!(writer, "            client: reqwest::Client::new(),")?;
        writeln!(writer, "            location: \"{}\".to_string(),", self.location)?;
        writeln!(writer, "            credentials,")?;
        writeln!(writer, "        }}")?;
        writeln!(writer, "    }}")?;

        // create a method for each operation
        for (operation_name, operation) in &self.binding.operations {
            write_async_soap_call(writer, operation_name, operation)?;
        }

        writeln!(writer, "}}")?;

        Ok(())
    }
}

fn write_async_soap_call<W>(writer: &mut W, operation_name: &str, operation: &SoapOperation) -> WriterResult<()>
where
    W: io::Write,
{
    // generate an async fn for the operation
    let rust_fn_name = to_snake_case(operation_name);
    let request_name = format!("{operation_name}InputEnvelope");
    let response_name = operation
        .output
        .as_ref()
        .map(|_| format!("{operation_name}OutputEnvelope"));

    if let Some(res_name) = response_name {
        writeln!(
            writer,
            "pub async fn {rust_fn_name}(&self, req: {request_name}) -> error::SoapResult<{res_name}> {{"
        )?;
    } else {
        writeln!(
            writer,
            "pub async fn {rust_fn_name}(&self, req: {request_name}) -> error::SoapResult<()> {{"
        )?;
    }

    writeln!(
        writer,
        "    let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));"
    )?;
    writeln!(
        writer,
        "    helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await"
    )?;

    writeln!(writer, "}}")?;
    Ok(())
}
