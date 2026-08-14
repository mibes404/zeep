use super::binding::{SoapBinding, SoapOperation};
use crate::{
    error::{WriterError, WriterResult},
    model::{TryFromNode, field::resolve_type},
    reader::WriteXml,
};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
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
        // the WSDL service name is used verbatim as a Rust type identifier, but WSDL
        // names are free-form XML tokens and may contain characters (e.g. `.`) that
        // are not valid in Rust identifiers, so sanitize it first
        let struct_name = sanitize_rust_ident(&self.name);

        // create a wrapping Rust struct for the service
        writeln!(writer, "pub struct {struct_name} {{")?;
        writeln!(writer, "    pub client: reqwest::Client,")?;
        writeln!(writer, "    pub location: String,")?;
        writeln!(writer, "    pub credentials: Option<(String, String)>,")?;
        writeln!(writer, "}}")?;

        // create an implementation for the service
        writeln!(writer, "impl {struct_name} {{")?;
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

        // create a default method
        writeln!(writer)?;
        writeln!(writer, "    pub fn default() -> Self {{")?;
        writeln!(writer, "        Self {{")?;
        writeln!(writer, "            client: reqwest::Client::new(),")?;
        writeln!(writer, "            location: \"{}\".to_string(),", self.location)?;
        writeln!(writer, "            credentials: None,")?;
        writeln!(writer, "        }}")?;
        writeln!(writer, "    }}")?;

        // create builder pattern methods
        writeln!(writer)?;
        writeln!(
            writer,
            "    pub fn with_client(mut self, client: reqwest::Client) -> Self {{"
        )?;
        writeln!(writer, "        self.client = client;")?;
        writeln!(writer, "        self")?;
        writeln!(writer, "    }}")?;

        writeln!(writer)?;
        writeln!(
            writer,
            "    pub fn with_location(mut self, location: impl Into<String>) -> Self {{"
        )?;
        writeln!(writer, "        self.location = location.into();")?;
        writeln!(writer, "        self")?;
        writeln!(writer, "    }}")?;

        writeln!(writer)?;
        writeln!(
            writer,
            "    pub fn with_credentials(mut self, credentials: (String, String)) -> Self {{"
        )?;
        writeln!(writer, "        self.credentials = Some(credentials);")?;
        writeln!(writer, "        self")?;
        writeln!(writer, "    }}")?;

        // create a method for each operation
        for (operation_name, operation) in &self.binding.operations {
            write_async_soap_call(writer, operation_name, operation)?;
        }

        writeln!(writer, "}}")?;

        Ok(())
    }
}

/// Sanitizes an arbitrary WSDL name (e.g. a `wsdl:service` name) into a valid Rust
/// identifier by replacing any character that isn't alphanumeric or `_` with `_`,
/// and prefixing with `_` if the result would otherwise start with a digit.
///
/// WSDL service/port names commonly embed a version number using a literal dot
/// (e.g. `RecherchePointV2.0`), which is not a valid Rust identifier character.
fn sanitize_rust_ident(name: &str) -> String {
    let mut sanitized: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect();

    if sanitized.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        sanitized.insert(0, '_');
    }

    sanitized
}

fn write_async_soap_call<W>(writer: &mut W, operation_name: &str, operation: &SoapOperation) -> WriterResult<()>
where
    W: io::Write,
{
    // generate an async fn for the operation. The envelope struct names generated
    // by binding/writer.rs are always pascal-cased (`{PascalCaseOperationName}Input/OutputEnvelope`),
    // so we must pascal-case the operation name here too, or we end up referencing
    // a type that doesn't exist (e.g. `rechercherPointInputEnvelope` instead of
    // `RechercherPointInputEnvelope`).
    let rust_fn_name = to_snake_case(operation_name);
    let operation_name = to_pascal_case(operation_name);
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

#[cfg(test)]
mod tests {
    use super::{sanitize_rust_ident, write_async_soap_call};
    use crate::model::{
        node::RustNode,
        soap::binding::{SoapEnvelope, SoapOperation},
        structures::RustType,
    };
    use std::rc::Rc;

    fn dummy_envelope() -> SoapEnvelope {
        SoapEnvelope {
            headers: vec![],
            body: Rc::new(RustNode {
                rust_type: RustType::Ignore,
                in_namespace: None,
            }),
        }
    }

    #[test]
    fn method_signature_uses_pascal_cased_envelope_names_matching_generated_structs() {
        // operation names come straight from the WSDL/portType and are not
        // necessarily pascal-cased (e.g. "rechercherPoint"), but the envelope
        // structs generated by binding/writer.rs always are
        // (e.g. "RechercherPointInputEnvelope").
        let operation = SoapOperation {
            action: None,
            input: dummy_envelope(),
            output: Some(dummy_envelope()),
        };

        let mut writer = Vec::new();
        write_async_soap_call(&mut writer, "rechercherPoint", &operation).unwrap();
        let output = String::from_utf8(writer).unwrap();

        assert!(
            output.contains("req: RechercherPointInputEnvelope"),
            "expected pascal-cased input envelope type, got: {output}"
        );
        assert!(
            output.contains("error::SoapResult<RechercherPointOutputEnvelope>"),
            "expected pascal-cased output envelope type, got: {output}"
        );
        assert!(
            !output.contains("rechercherPointInputEnvelope"),
            "should not reference the un-pascal-cased envelope name, got: {output}"
        );
    }

    #[test]
    fn sanitizes_dots_in_service_name_to_underscores() {
        assert_eq!(sanitize_rust_ident("RecherchePointV2.0"), "RecherchePointV2_0");
    }

    #[test]
    fn sanitizes_other_invalid_ident_characters() {
        assert_eq!(sanitize_rust_ident("My-Service Name!"), "My_Service_Name_");
    }

    #[test]
    fn leaves_already_valid_identifiers_untouched() {
        assert_eq!(sanitize_rust_ident("WeatherService"), "WeatherService");
    }

    #[test]
    fn prefixes_leading_digit_with_underscore() {
        assert_eq!(sanitize_rust_ident("2FastService"), "_2FastService");
    }
}
