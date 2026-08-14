//! This module contains the `XmlReader` struct which is responsible for reading the XML and
//! converting it to a `RustDocument` The entry point is the `read_xml` method which takes a
//! `FilesToRead` struct and returns a `RustDocument` The `Files` struct is used to hold the
//! XML content and the `FilesToRead` struct is used to hold the `Files` and the starting file
//! to read from.
//!
//! The `RustDocument` (and the included elements) implement the `WriteXml` trait which is used
//! to write the Rust code to a writer.
//!
//! The `read_input_file_and_xsd_files_at_path` helper function can be used to easily construct
//! the `FilesToRead` struct from a file path.

use crate::{
    error::{WriterError, WriterResult},
    model::{
        TryFromNode,
        doc::RustDocument,
        node::RustNode,
        soap::{binding::SoapBinding, message::SoapMessage, port::SoapPort, service::SoapService},
    },
};
use roxmltree::Node;
use std::{
    collections::HashMap,
    fmt::Display,
    io,
    path::{Component, Path, PathBuf},
};

pub const WELL_KNOWN_NAMESPACES: &[&str] = &[
    "http://www.w3.org/XML/1998/namespace",
    "http://www.w3.org/2001/XMLSchema",
    "http://www.w3.org/2001/XMLSchema-instance",
    "http://www.w3.org/2007/XMLSchema-versioning",
];

/// The `WriteXml` trait is used to write the Rust code to a writer, like a file or a buffer.
pub trait WriteXml<W>
where
    W: io::Write,
{
    /// Generate XML from the Rust type
    ///
    /// # Errors
    /// Returns an error if the XML could not be written to the writer
    fn write_xml(&self, writer: &mut W) -> WriterResult<()>;
}

/// The `XmlReader` struct is responsible for reading the XML and converting it to a `RustDocument`
pub struct XmlReader;

/// The `FileContent` struct is used to hold the XML content and a flag to indicate if the file
/// has been processed or not.
struct FileContent {
    xml: String,
    processed: bool,
}

impl FileContent {
    #[must_use]
    fn new(xml: String) -> Self {
        FileContent { xml, processed: false }
    }
}

type Schemalocation = String;

/// The `Files` struct is used to hold the XML content
pub struct Files {
    map: HashMap<Schemalocation, FileContent>,
}

impl Files {
    pub fn new<F, X>(file_name: F, xml: X) -> Self
    where
        F: Display,
        X: Display,
    {
        Files {
            map: HashMap::from([(file_name.to_string(), FileContent::new(xml.to_string()))]),
        }
    }

    pub fn add<F, X>(&mut self, file_name: F, xml: X)
    where
        F: Display,
        X: Display,
    {
        let Files { map, .. } = self;
        map.insert(file_name.to_string(), FileContent::new(xml.to_string()));
    }

    /// Resolve `schema_location`, relative to the file that imports it (`from_file`), against the
    /// files already loaded in memory. If it is not found there, try to load it from disk. This is
    /// what makes `../../some/other/dir/file.xsd`-style `schemaLocation` paths in imports work,
    /// without having to pre-scan the whole project tree up front.
    fn resolve(&mut self, from_file: &str, schema_location: &str) -> WriterResult<String> {
        let resolved = normalize_path(from_file, schema_location);

        if !self.map.contains_key(&resolved) {
            let xml = std::fs::read_to_string(&resolved)
                .map_err(|_| WriterError::ImportNotFound(schema_location.to_string()))?;
            self.map.insert(resolved.clone(), FileContent::new(xml));
        }

        Ok(resolved)
    }
}

/// Join `relative` onto the directory of `base`, then lexically collapse any `.` and `..`
/// components. This deliberately does not touch the filesystem (no `canonicalize`), so it also
/// works for the in-memory files used in tests.
fn normalize_path(base: &str, relative: &str) -> String {
    let base_dir = Path::new(base).parent().unwrap_or_else(|| Path::new(""));
    let joined = base_dir.join(relative);

    let mut normalized = PathBuf::new();
    for component in joined.components() {
        match component {
            Component::ParentDir => {
                if !normalized.pop() {
                    normalized.push("..");
                }
            }
            Component::CurDir => {}
            other => normalized.push(other),
        }
    }

    normalized.to_string_lossy().into_owned()
}

/// The `FilesToRead` struct is used to hold the `Files` and the starting file to read from.
pub struct FilesToRead {
    start_with_file: String,
    files: Files,
}

impl FilesToRead {
    pub fn new(start_with_file: impl Display, files: Files) -> Self {
        FilesToRead {
            start_with_file: start_with_file.to_string(),
            files,
        }
    }

}

impl XmlReader {
    /// Read the XML and convert it to a `RustDocument`
    ///
    /// # Errors
    /// Returns an error if the XSD/WSDL is invalid
    pub fn read_xml(files_to_read: &mut FilesToRead) -> WriterResult<RustDocument> {
        let FilesToRead { start_with_file, files } = files_to_read;
        Self::read_xml_internal(start_with_file, files)
    }

    #[cfg(test)]
    pub(crate) fn read_xml_from_file(file_name: &str, xml: &str) -> WriterResult<RustDocument> {
        let files = Files::new(file_name, xml);
        let mut files_to_read = FilesToRead::new(file_name, files);
        Self::read_xml(&mut files_to_read)
    }

    fn read_xml_internal(file_name: &str, files: &mut Files) -> WriterResult<RustDocument> {
        let already_processed = files.map.get(file_name).is_some_and(|f| f.processed);
        if already_processed {
            return Ok(RustDocument::empty());
        }

        let xml = files
            .map
            .get(file_name)
            .ok_or_else(|| WriterError::ImportNotFound(file_name.to_string()))?
            .xml
            .clone();

        let doc = roxmltree::Document::parse(&xml)
            .map_err(|e| WriterError::new(format!("Unable to parse file {file_name}: {e}")))?;
        let mut rust_doc = RustDocument::init(&doc);

        for child in doc.root().children() {
            Self::read(child, file_name, files, &mut rust_doc)?;
        }

        if let Some(file) = files.map.get_mut(file_name) {
            file.processed = true;
        }

        Ok(rust_doc)
    }

    fn read<'n>(node: Node<'n, 'n>, file_name: &str, files: &mut Files, doc: &mut RustDocument) -> WriterResult<()> {
        if !node.is_element() {
            return Ok(());
        }

        if let Some(target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(target_namespace);
        }

        match node.tag_name().name() {
            "definitions" => Self::read_wsdl(node, file_name, files, doc)?,
            "schema" => Self::read_xsd(node, file_name, files, doc)?,
            _ => return Ok(()),
        }

        Ok(())
    }

    fn read_wsdl<'n>(
        node: Node<'n, 'n>,
        file_name: &str,
        files: &mut Files,
        doc: &mut RustDocument,
    ) -> WriterResult<()> {
        for child in node.children() {
            let node_name = child.tag_name().name();
            // first read the types as if it were an XSD
            if node_name == "types" {
                Self::read_soap_types_schema(file_name, files, doc, child)?;
            }

            // read soap messages
            if node_name == "message" {
                let message = SoapMessage::try_from_node(child, doc)?;
                doc.soap_messages.push(message.into());
            }

            // read soap ports
            if node_name == "portType" {
                let port = SoapPort::try_from_node(child, doc)?;
                doc.soap_ports.push(port.into());
            }

            // read soap bindings
            if node_name == "binding" {
                let binding = SoapBinding::try_from_node(child, doc)?;
                doc.soap_bindings.push(binding.into());
            }

            // read soap services
            if node_name == "service" {
                let service = SoapService::try_from_node(child, doc)?;
                doc.soap_services.push(service);
            }
        }

        Ok(())
    }

    fn read_soap_types_schema<'n>(
        file_name: &str,
        files: &mut Files,
        doc: &mut RustDocument,
        child: Node<'n, 'n>,
    ) -> Result<(), WriterError> {
        let mut any = false;
        for node in child.children() {
            if node.tag_name().name() == "schema" {
                Self::read_xsd(node, file_name, files, doc)?;
                any = true;
            }
        }
        if any { Ok(()) } else { Err(WriterError::SchemaNotFound) }
    }

    fn read_xsd<'n>(
        node: Node<'n, 'n>,
        file_name: &str,
        files: &mut Files,
        doc: &mut RustDocument,
    ) -> WriterResult<()> {
        // Switch to the schema's target namespace if it has one
        if let Some(target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(target_namespace);
            doc.set_form_defaults(
                target_namespace,
                node.attribute("elementFormDefault") == Some("qualified"),
                node.attribute("attributeFormDefault") == Some("qualified"),
            );
        }

        for child in node.children() {
            if child.tag_name().name() == "import" {
                doc.extend(Self::process_import(child, file_name, files)?);
                continue;
            }

            if let Ok(child_node) = RustNode::try_from_node(child, doc) {
                doc.nodes.push(child_node.into());
            }
        }

        Ok(())
    }

    fn process_import(node: Node, file_name: &str, files: &mut Files) -> WriterResult<RustDocument> {
        let namespace = node.attribute("namespace").ok_or(WriterError::NamespaceMissing)?;

        if WELL_KNOWN_NAMESPACES.contains(&namespace) {
            return Ok(RustDocument::empty());
        }

        let Some(schema_location) = node.attribute("schemaLocation") else {
            return Ok(RustDocument::empty());
        };

        // Resolve schemaLocation relative to the file that imports it (this is what makes
        // `../../some/other/dir/file.xsd`-style imports work), loading it from disk on demand
        // if it isn't already known.
        let resolved = files.resolve(file_name, schema_location)?;

        Self::read_xml_internal(&resolved, files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        field::RustFieldType,
        structures::{RustType, complex::ComplexProps, simple::SimpleProps},
    };

    #[test]
    fn can_read_a_simple_xsd() {
        const XSD: &str = include_str!("../test-data/single-complex.xsd");
        let mut files = Files::new("types.xsd", XSD);
        let nodes = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap().nodes;
        assert_eq!(nodes.len(), 1);
        let node = nodes.first().unwrap();
        let RustType::Complex(props) = &node.rust_type else {
            panic!()
        };

        let ComplexProps {
            fields,
            xml_name,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "InstalledAppType");
        assert_eq!(fields.len(), 14);
        let id_field = fields.first().unwrap();
        assert_eq!(id_field.xml_name, "Id");
        assert_eq!(id_field.rust_type, RustFieldType::String);
        assert!(id_field.is_optional);
        assert!(!id_field.is_vec);

        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
        assert_eq!(target_namespace.as_ref().unwrap().abbreviation, "typ");
    }

    #[test]
    fn can_read_nested_target_namespace() {
        const XSD_MESSAGES: &str = include_str!("../test-data/single-simple-with-nested-tns.xsd");
        const XSD_TYPES: &str = include_str!("../test-data/single-complex.xsd");
        let mut files = Files::new("messages.xsd", XSD_MESSAGES);
        files.add("types.xsd", XSD_TYPES);

        let nodes = XmlReader::read_xml_internal("messages.xsd", &mut files).unwrap().nodes;
        assert_eq!(nodes.len(), 2);

        let type_node = nodes.first().unwrap();
        let RustType::Complex(props) = &type_node.rust_type else {
            panic!()
        };

        let ComplexProps {
            xml_name,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "InstalledAppType");
        // check that a target namespace was set
        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );

        let message_node = nodes.last().unwrap();
        let RustType::Simple(props) = &message_node.rust_type else {
            panic!()
        };
        let SimpleProps {
            xml_name,
            rust_type,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "ResponseCodeType");
        assert_eq!(*rust_type, RustFieldType::String);
        // check that a target namespace was set
        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/messages"
        );
    }

    #[test]
    fn locally_declared_fields_are_unqualified_unless_opted_in() {
        // Regression test: per the XSD spec, elementFormDefault/attributeFormDefault
        // default to "unqualified" when absent from <xs:schema>, so locally-declared
        // fields (nested inside a complexType) must not carry a namespace prefix
        // unless the schema opts in via elementFormDefault/attributeFormDefault, or
        // the field itself has an explicit `form="qualified"` override.
        const XSD: &str = include_str!("../test-data/unqualified-form-default.xsd");
        let mut files = Files::new("types.xsd", XSD);
        let nodes = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap().nodes;
        assert_eq!(nodes.len(), 1);
        let node = nodes.first().unwrap();
        let RustType::Complex(props) = &node.rust_type else {
            panic!()
        };

        let ComplexProps { fields, .. } = &**props;
        assert_eq!(fields.len(), 3);

        let id_field = fields.iter().find(|f| f.xml_name == "Id").unwrap();
        assert!(id_field.target_namespace.is_none());

        let attr_field = fields.iter().find(|f| f.xml_name == "Attr").unwrap();
        assert!(attr_field.target_namespace.is_none());

        // Explicit `form="qualified"` overrides the schema-level default.
        let qualified_field = fields.iter().find(|f| f.xml_name == "Qualified").unwrap();
        assert_eq!(
            qualified_field.target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
    }

    #[test]
    fn locally_declared_fields_are_qualified_when_form_default_is_qualified() {
        // single-complex.xsd sets elementFormDefault="qualified", so locally-declared
        // elements should inherit the schema's target namespace.
        const XSD: &str = include_str!("../test-data/single-complex.xsd");
        let mut files = Files::new("types.xsd", XSD);
        let nodes = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap().nodes;
        let node = nodes.first().unwrap();
        let RustType::Complex(props) = &node.rust_type else {
            panic!()
        };

        let ComplexProps { fields, .. } = &**props;
        let id_field = fields.iter().find(|f| f.xml_name == "Id").unwrap();
        assert_eq!(
            id_field.target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
    }

    #[test]
    fn normalize_path_resolves_relative_imports() {
        assert_eq!(normalize_path("services/foo.wsdl", "bar.xsd"), "services/bar.xsd");
        assert_eq!(
            normalize_path("a/b/c/foo.wsdl", "../../x/y/bar.xsd"),
            "a/x/y/bar.xsd"
        );
        assert_eq!(normalize_path("a/b/foo.wsdl", "./bar.xsd"), "a/b/bar.xsd");
    }

    #[test]
    fn can_read_import_that_climbs_out_of_its_directory() {
        // Regression test: schemaLocation paths like `../../Dictionnaires/v5.0/foo.xsd` (as used
        // by real-world WSDLs, e.g. Enedis SGE) must resolve relative to the importing file's own
        // directory, not the entry file's directory.
        const XSD_TYPES: &str = include_str!("../test-data/single-complex.xsd");
        const XSD_MESSAGES_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
    targetNamespace="http://schemas.microsoft.com/exchange/services/2006/messages"
    xmlns:typ="http://schemas.microsoft.com/exchange/services/2006/types">
    <xs:import namespace="http://schemas.microsoft.com/exchange/services/2006/types"
        schemaLocation="../../dictionaries/v5.0/types.xsd"/>
    <xs:element name="ResponseCode" type="xs:string"/>
</xs:schema>"#;

        let mut files = Files::new("services/exchange/messages.xsd", XSD_MESSAGES_TEMPLATE);
        files.add("dictionaries/v5.0/types.xsd", XSD_TYPES);

        let nodes = XmlReader::read_xml_internal("services/exchange/messages.xsd", &mut files)
            .unwrap()
            .nodes;

        // One node from the imported types.xsd, one from messages.xsd itself.
        assert_eq!(nodes.len(), 2);
        assert!(nodes.iter().any(|n| n.rust_type.xml_name() == Some("InstalledAppType")));
        assert!(nodes.iter().any(|n| n.rust_type.xml_name() == Some("ResponseCode")));
    }

    #[test]
    fn can_read_elements_with_extensions() {
        const XSD_TYPES: &str = include_str!("../test-data/extensions.xsd");
        let mut files = Files::new("types.xsd", XSD_TYPES);

        let rust_doc = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap();

        // check that we found the two namespaces
        assert_eq!(rust_doc.namespaces.len(), 2, "Expected two namespaces");

        let nodes = &rust_doc.nodes;
        assert_eq!(nodes.len(), 5);

        let type_node = nodes.get(4).expect("Expected a fourth node");
        let RustType::Complex(props) = &type_node.rust_type else {
            panic!()
        };

        let ComplexProps {
            xml_name,
            target_namespace,
            ..
        } = &**props;

        assert_eq!(xml_name, "ItemChangeDescriptionType");
        // check that a target namespace was set
        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );

        let message_node = nodes.last().unwrap();
        let RustType::Complex(props) = &message_node.rust_type else {
            panic!()
        };
        let ComplexProps { fields, .. } = &**props;

        // check the number of fields, it should include the fields from the base type
        assert_eq!(fields.len(), 4);

        // check the field names
        let field_names: Vec<&str> = fields.iter().map(|f| f.xml_name.as_str()).collect();
        assert_eq!(field_names, ["Path", "Source", "Target", "Drive"]);

        // the Target and Drive should be optional
        let target_field = fields.iter().find(|f| f.xml_name == "Target").unwrap();
        assert!(target_field.is_optional);

        let drive_field = fields.iter().find(|f| f.xml_name == "Drive").unwrap();
        assert!(drive_field.is_optional);
    }

    #[test]
    fn can_parse_forward_pointing_base_type() {
        const XSD_TYPES: &str = include_str!("../test-data/forward-pointing-type.xsd");
        let mut files = Files::new("types.xsd", XSD_TYPES);

        let rust_doc = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap();
        assert_eq!(rust_doc.nodes.len(), 2);

        // check node name
        let node = rust_doc.nodes.first().unwrap();
        assert_eq!(node.rust_type.xml_name(), Some("AddDelegateType"));
    }

    #[test]
    fn can_parse_groups() {
        const XSD_TYPES: &str = include_str!("../test-data/use-of-groups.xsd");
        let mut files = Files::new("types.xsd", XSD_TYPES);

        let rust_doc = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap();
        assert_eq!(rust_doc.nodes.len(), 3);

        // check node name
        let node = rust_doc.nodes.last().unwrap();
        assert_eq!(node.rust_type.xml_name(), Some("TimeChangeType"));
    }

    #[test]
    fn should_ensure_that_we_have_namespaces_on_all_types() {
        const XSD_MESSAGES: &str = include_str!("../test-data/exchange/messages.xsd");
        const XSD_TYPES: &str = include_str!("../test-data/exchange/types.xsd");
        let mut files = Files::new("messages.xsd", XSD_MESSAGES);
        files.add("types.xsd", XSD_TYPES);

        let nodes = XmlReader::read_xml_internal("messages.xsd", &mut files).unwrap().nodes;
        assert_eq!(nodes.len(), 1457);

        // get the GetUserAvailabilityRequestType
        let node = nodes
            .iter()
            .find(|n| n.rust_type.xml_name() == Some("GetUserAvailabilityRequestType"))
            .expect("Expected GetUserAvailabilityRequestType");

        // the node should have 4 fields. TimeZone, FreeBusyViewOptions and SuggestionsViewOptions should have the "typ" prefix
        // the MailboxDataArray should have the "mes" prefix

        let RustType::Complex(props) = &node.rust_type else {
            panic!()
        };

        assert_eq!(props.fields.len(), 4);

        // get the TimeZone field
        let time_zone = props.fields.first().expect("Expected a TimeZone field");
        assert_eq!(time_zone.xml_name, "TimeZone");
        assert_eq!(time_zone.target_namespace.as_ref().unwrap().abbreviation, "typ");

        // get the MailboxDataArray field
        let mailbox_data_array = props.fields.get(1).expect("Expected a MailboxDataArray field");
        assert_eq!(mailbox_data_array.xml_name, "MailboxDataArray");
        assert_eq!(
            mailbox_data_array.target_namespace.as_ref().unwrap().abbreviation,
            "mes"
        );

        // get the FreeBusyViewOptions field
        let free_busy_view_options = props.fields.get(2).expect("Expected a FreeBusyViewOptions field");
        assert_eq!(free_busy_view_options.xml_name, "FreeBusyViewOptions");
        assert_eq!(
            free_busy_view_options.target_namespace.as_ref().unwrap().abbreviation,
            "typ"
        );

        // get the SuggestionsViewOptions field
        let suggestions_view_options = props.fields.last().expect("Expected a SuggestionsViewOptions field");
        assert_eq!(suggestions_view_options.xml_name, "SuggestionsViewOptions");
        assert_eq!(
            suggestions_view_options.target_namespace.as_ref().unwrap().abbreviation,
            "typ"
        );
    }

    #[test]
    fn can_handle_inline_namespace_definitions() {
        const XSD_TYPES: &str = include_str!("../test-data/inline-namespace-definition.xsd");
        let mut files = Files::new("types.xsd", XSD_TYPES);

        let rust_doc = XmlReader::read_xml_internal("types.xsd", &mut files).unwrap();
        assert_eq!(rust_doc.nodes.len(), 2);

        let greeting_request = rust_doc.nodes.get(1).unwrap();
        let RustType::Complex(props) = &greeting_request.rust_type else {
            panic!()
        };

        assert_eq!(props.fields.len(), 1);
        let greeting_field = props.fields.first().unwrap();
        assert_eq!(greeting_field.xml_name, "Body");
    }

    #[test]
    fn can_read_wsdl_file_with_service_security_header() {
        const WSDL: &str = include_str!("../test-data/claim_service.wsdl");
        let mut files = Files::new("claim_service.wsdl", WSDL);
        let rust_doc = XmlReader::read_xml_internal("claim_service.wsdl", &mut files).unwrap();

        // The WSDL has many elements and complex types
        assert_eq!(rust_doc.nodes.len(), 78);

        // Verify that GetClaim element exists
        let get_claim_node = rust_doc
            .nodes
            .iter()
            .find(|n| n.rust_type.xml_name() == Some("GetClaim"))
            .expect("GetClaim element should exist");
        assert_eq!(get_claim_node.rust_type.xml_name(), Some("GetClaim"));

        // Verify that ServiceSecurityHeader exists (the element that was causing the error)
        let security_header_node = rust_doc
            .nodes
            .iter()
            .find(|n| n.rust_type.xml_name() == Some("ServiceSecurityHeader"))
            .expect("ServiceSecurityHeader element should exist");
        assert_eq!(security_header_node.rust_type.xml_name(), Some("ServiceSecurityHeader"));

        // Verify that SOAP messages were read (including header messages)
        assert!(!rust_doc.soap_messages.is_empty());

        // Verify the header message for GetClaim exists
        let header_message = rust_doc
            .soap_messages
            .iter()
            .find(|m| m.xml_name == "GetClaimServiceSecurityHeader")
            .expect("GetClaimServiceSecurityHeader message should exist");
        assert!(header_message.parts.contains_key("ServiceSecurityHeader"));

        // Verify that SOAP bindings were read
        assert_eq!(rust_doc.soap_bindings.len(), 2); // soap and soap12 bindings

        // Verify that the binding operations have headers
        let binding = &rust_doc.soap_bindings[0];
        let get_claim_operation = binding
            .operations
            .get("GetClaim")
            .expect("GetClaim operation should exist");
        assert_eq!(get_claim_operation.input.headers.len(), 1);
        assert_eq!(get_claim_operation.input.headers[0].0, "ServiceSecurityHeader");
    }

    #[test]
    fn can_read_wsdl_with_headers_in_same_message() {
        // Exchange WSDL pattern: headers and body parts in the same message
        const WSDL: &str = include_str!("../test-data/headers_in_same_message.wsdl");

        let rust_doc = XmlReader::read_xml_from_file("test.wsdl", WSDL).unwrap();

        // Check that the message was read with both parts
        let test_message = rust_doc
            .soap_messages
            .iter()
            .find(|m| m.xml_name == "TestSoapIn")
            .expect("TestSoapIn message should exist");

        assert_eq!(test_message.parts.len(), 2, "Message should have 2 parts");
        assert!(
            test_message.parts.contains_key("parameters"),
            "Should have 'parameters' part"
        );
        assert!(
            test_message.parts.contains_key("TestHeader"),
            "Should have 'TestHeader' part"
        );

        // Check that the binding was read with the header
        assert_eq!(rust_doc.soap_bindings.len(), 1);
        let binding = &rust_doc.soap_bindings[0];
        let operation = binding
            .operations
            .get("TestOperation")
            .expect("TestOperation should exist");

        assert_eq!(operation.input.headers.len(), 1, "Operation should have 1 header");
        assert_eq!(operation.input.headers[0].0, "TestHeader");
    }

    #[test]
    fn can_handle_multi_namespace_wsdl() {
        const WSDL: &str = include_str!("../test-data/multi_namespace.wsdl");
        let mut files = Files::new("multi_namespace.wsdl", WSDL);
        let rust_doc = XmlReader::read_xml_internal("multi_namespace.wsdl", &mut files).unwrap();

        // Verify all nodes were read (2 elements from first schema, 3 types from second schema)
        assert_eq!(rust_doc.nodes.len(), 5);

        // Verify first namespace (security service)
        let security_ns = rust_doc
            .target_namespaces
            .iter()
            .find(|ns| ns.namespace == "http://services.lighthouse1.com/services/security/")
            .expect("Security namespace should exist");
        assert_eq!(security_ns.abbreviation, "sec1");

        // Verify second namespace (security messages)
        let messages_ns = rust_doc
            .target_namespaces
            .iter()
            .find(|ns| ns.namespace == "http://services.lighthouse1.com/SecurityService/SecurityMessages.xsd")
            .expect("SecurityMessages namespace should exist");
        assert_eq!(messages_ns.abbreviation, "sec");

        // Verify nodes are in correct namespaces
        let login_by_token = rust_doc
            .nodes
            .iter()
            .find(|n| n.rust_type.xml_name() == Some("LoginByToken"))
            .expect("LoginByToken should exist");
        assert_eq!(
            login_by_token.in_namespace.as_ref().unwrap().namespace,
            "http://services.lighthouse1.com/services/security/"
        );

        let login_response = rust_doc
            .nodes
            .iter()
            .find(|n| n.rust_type.xml_name() == Some("LoginResponseByToken"))
            .expect("LoginResponseByToken should exist");
        assert_eq!(
            login_response.in_namespace.as_ref().unwrap().namespace,
            "http://services.lighthouse1.com/SecurityService/SecurityMessages.xsd"
        );

        // Test that generated code includes all referenced namespaces in yaserde annotations
        let mut generated_code: Vec<u8> = Vec::new();
        rust_doc.write_xml(&mut generated_code).unwrap();
        let code_str = String::from_utf8(generated_code).unwrap();

        // LoginByTokenResponse should have both sec1 and sec namespaces in its yaserde annotation
        // because it references a field from the sec namespace
        let login_by_token_response_start = code_str
            .find("pub struct LoginByTokenResponse")
            .expect("LoginByTokenResponse not found");
        let yaserde_attr_start = code_str[..login_by_token_response_start]
            .rfind("#[yaserde(")
            .expect("yaserde attribute not found");
        let yaserde_attr_end = login_by_token_response_start;
        let yaserde_attr = &code_str[yaserde_attr_start..yaserde_attr_end];

        // Check that both namespaces are present
        assert!(
            yaserde_attr.contains(r#""sec1" = "http://services.lighthouse1.com/services/security/""#),
            "yaserde should include sec1 namespace (struct's own namespace)"
        );
        assert!(
            yaserde_attr.contains(r#""sec" = "http://services.lighthouse1.com/SecurityService/SecurityMessages.xsd""#),
            "yaserde should include sec namespace because LoginByTokenResult field references it"
        );
    }
}
